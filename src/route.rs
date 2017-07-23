extern crate rand;

use ::std::mem;
use ::rocket::State;
use ::rocket::request::LenientForm;
use ::rocket_contrib::Json;
use ::airport::{self, Airport, ICAO};
use self::rand::Rng;

macro_rules! enum_with_form_parser {
    (name = $name:ident, form = $form_ty:ty,
        $($struct_variant:ident => $enum_variant:ident($type:ty),)*) => {

        #[derive(Debug)]
        enum $name {
            $($enum_variant($type),)*
        }

        impl $name {
            fn from_form(form: &$form_ty) -> Vec<$name> {
                let mut found = Vec::new();

                $(match form.$struct_variant {
                    Some(ref v) => found.push($name::$enum_variant(v.clone())),
                    None => (),
                })*

                found
            }
        }
    };
}

#[derive(FromForm, Debug, Clone)]
pub struct FilterForm {
    mach: f32,
    dep_icao: Option<ICAO>,
    dep_type: Option<airport::Type>,
    arr_icao: Option<ICAO>,
    arr_type: Option<airport::Type>,
}

enum_with_form_parser!(
    name = AirportFilter,
    form = FilterForm,
        dep_type => Type(airport::Type),
);

enum_with_form_parser!(
    name = RouteFilter,
    form = FilterForm,
        arr_type => ArrType(airport::Type),
);

#[derive(Debug, Serialize)]
pub struct Route {
    pub departure: Airport,
    pub arrival:   Airport,
    pub distance:  f32,
}

impl Route {
    pub fn create(departure: Airport, arrival: Airport) -> Route {
        let mut route = Route {
            departure,
            arrival,
            distance: 0.0,
        };

        route.distance = route.calculate_distance();
        route
    }

    pub fn calculate_distance(&self) -> f32 {
        let radius = 3440.; // Earth's radius in nautical miles

        let lat1 = self.departure.pos.lat.to_radians();
        let lat2 = self.arrival.pos.lat.to_radians();
        let lat  = lat2 - lat1;
        let lon  = (self.arrival.pos.lon - self.departure.pos.lon).to_radians();

        let a = (lat / 2.0).sin().powi(2) + lat1.cos() * lat2.cos() * (lon / 2.0).sin().powi(2);
        let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

        radius * c
    }
}

fn airport_from_icao<'a>(icao: &str, airports: &'a [Airport]) -> Option<&'a Airport> {
    airports.iter().find(|&airport| airport.icao == icao)
}

fn find_departure<'a>(form: &FilterForm, airports: &'a [Airport])
    -> Option<&'a Airport> {

    if let Some(ICAO(ref icao)) = form.dep_icao {
        airport_from_icao(icao, &airports)
    } else {
        let filters = AirportFilter::from_form(&form);

        let found = airports.iter()
            .filter(|&airport| {
                filters.iter().any(|ref filter| {
                    use self::AirportFilter::*;

                    match **filter {
                        Type(ref _type) => airport._type == *_type,
                    }
                })
            })
            .collect::<Vec<_>>();

        if found.len() > 0 {
            Some(found[rand::thread_rng().gen_range(0, found.len())])
        } else {
            None
        }
    }
}

// TODO: implement error handling
#[post("/filter", data = "<form>")]
pub fn filter_routes(form: LenientForm<FilterForm>, airports: State<Vec<Airport>>)
    -> Json<Vec<Route>> {

    let mut form = form.into_inner();

    if let (Some(ICAO(dep_icao)), Some(ICAO(arr_icao)))
        = (form.dep_icao.clone(), form.arr_icao.clone()) {

        let dep = airport_from_icao(&dep_icao, &airports).unwrap();
        let arr = airport_from_icao(&arr_icao, &airports).unwrap();

        Json(vec![Route::create(dep.clone(), arr.clone())])
    } else {
        // If the arrival is set but the departure is not, we flip the arrival and departure
        // to imitate filtering for just the arrival airport
        let flipped_dep = form.dep_icao.is_none() && form.arr_icao.is_some();

        if flipped_dep {
            mem::swap(&mut form.dep_icao, &mut form.arr_icao);
            mem::swap(&mut form.dep_type, &mut form.arr_type);
        }

        let filters   = RouteFilter::from_form(&form);
        let departure = find_departure(&form, &airports).unwrap();

        let mut routes = airports.iter()
            .filter(|airport| {
                filters.iter().any(|ref filter| {
                    use self::RouteFilter::*;
                    
                    match **filter {
                        ArrType(ref _type) => airport._type == *_type,
                    }
                })
            })
            .map(|arrival| {
                if flipped_dep {
                    Route::create(arrival.clone(), departure.clone())
                } else {
                    Route::create(departure.clone(), arrival.clone())
                }
            })
            .collect::<Vec<_>>();

        routes.sort_by_key(|route| route.distance as i32);
        routes = routes.into_iter()
                       .take(30)
                       .collect::<Vec<_>>();

        Json(routes)
    }
}