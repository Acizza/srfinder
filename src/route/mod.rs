extern crate rand;
extern crate rayon;
extern crate time;

mod form;

use ::rocket::State;
use ::rocket::response::status::NotFound;
use ::rocket::request::LenientForm;
use ::rocket_contrib::Json;
use ::airport::{Airport, ICAO};
use self::rand::Rng;
use self::rayon::prelude::*;
use self::form::{AirportFilter, RouteFilter, FilterForm, Time};
use self::time::PreciseTime;

const KNOTS_PER_MACH: f32 = 666.739;

#[derive(Debug, Serialize)]
pub struct Route {
    pub departure: Airport,
    pub arrival:   Airport,
    pub distance:  f32,
    pub time:      f32,
}

impl Route {
    pub fn create(departure: Airport, arrival: Airport, mach: f32) -> Route {
        let mut route = Route {
            departure,
            arrival,
            distance: 0.0,
            time:     0.0,
        };

        route.distance = route.calculate_distance();
        route.time     = route.calculate_time(mach);
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

    pub fn calculate_time(&mut self, mach: f32) -> f32 {
        self.distance / (mach * KNOTS_PER_MACH)
    }

    pub fn matches_filters(&self, filters: &[RouteFilter]) -> bool {
        let matches = filters.iter().all(|ref filter| {
            use self::RouteFilter::*;
            
            match **filter {
                ArrType(ref _type)       => self.arrival._type == *_type,
                ArrRunwayLength(ref len) => len.any_match(&self.arrival.runways),
                ArrCountry(ref country)  => self.arrival.region.country == country.as_str(),
                MinTime(Time(min_time))  => self.time >= min_time,
                MaxTime(Time(max_time))  => self.time <= max_time,
            }
        });

        matches && self.arrival.icao != self.departure.icao
    }

    pub fn find_all(form: &FilterForm, airports: &[Airport])
        -> Result<Vec<Route>, NotFound<String>> {

        let departure = airports.find_by_form(&form).ok_or(
            NotFound("departure airport not found".into()))?;

        let filters = RouteFilter::from_form(&form);

        let mut routes = airports.par_iter()
            .filter_map(|arrival| {
                let route = Route::create(departure.clone(),
                                          arrival.clone(),
                                          form.mach);

                if route.matches_filters(&filters) {
                    Some(route)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        routes.sort_by_key(|route| route.distance as i32);

        Ok(routes.into_iter()
              .take(100)
              .collect())
    }
}

trait FindAirport<'a> {
    fn find_by_icao(&self, icao: &str) -> Option<&'a Airport>;
    fn find_by_form(&self, form: &FilterForm) -> Option<&'a Airport>;
    fn find_by_filters(&self, filters: &[AirportFilter]) -> Vec<&'a Airport>;
}

impl<'a> FindAirport<'a> for &'a [Airport] {
    fn find_by_icao(&self, icao: &str) -> Option<&'a Airport> {
        self.iter().find(|&a| a.icao == icao)
    }

    fn find_by_form(&self, form: &FilterForm) -> Option<&'a Airport> {
        match form.dep_icao {
            Some(ICAO(ref icao)) => self.find_by_icao(icao),
            None => {
                let filters = AirportFilter::from_form(&form);
                let found   = self.find_by_filters(&filters);

                if found.len() > 0 {
                    Some(found[rand::thread_rng().gen_range(0, found.len())])
                } else {
                    None
                }
            }
        }
    }

    fn find_by_filters(&self, filters: &[AirportFilter]) -> Vec<&'a Airport> {
        self.iter().filter(|&airport| {
            filters.iter().all(|ref filter| {
                use self::AirportFilter::*;

                match **filter {
                    Type(ref _type)       => airport._type == *_type,
                    RunwayLength(ref len) => len.any_match(&airport.runways),
                    Country(ref country)  => airport.region.country == country.as_str(),
                }
            })
        })
        .collect()
    }
}

#[post("/filter", data = "<form>")]
pub fn filter_routes(form: LenientForm<FilterForm>, airports: State<Vec<Airport>>)
    -> Result<Json<Vec<Route>>, NotFound<String>> {

    let form     = form.into_inner();
    let airports = airports.inner().as_slice();

    if let (Some(ICAO(dep_icao)), Some(ICAO(arr_icao)))
        = (form.dep_icao.clone(), form.arr_icao.clone()) {

        let dep = airports.find_by_icao(&dep_icao).ok_or(
            NotFound("departure airport not found".into()))?;

        let arr = airports.find_by_icao(&arr_icao).ok_or(
            NotFound("arrival airport not found".into()))?;

        Ok(Json(vec![Route::create(dep.clone(), arr.clone(), form.mach)]))
    } else {
        let start_time = PreciseTime::now();
        let routes     = Route::find_all(&form, &airports)?;
        let end_time   = PreciseTime::now();
        println!("filtering time: {} ms", start_time.to(end_time) * 1000);

        Ok(Json(routes))
    }
}