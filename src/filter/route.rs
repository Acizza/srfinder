extern crate rayon;

use filter::{RouteFilter, DataForm, Time};
use filter::airport::{Airport, AirportSearching};
use self::rayon::prelude::*;

error_chain! {}

const KNOTS_PER_MACH: f32 = 666.739;

#[derive(Debug, Serialize)]
pub struct Route<'a> {
    pub departure: &'a Airport,
    pub arrival:   &'a Airport,
    pub distance:  f32,
    pub time:      f32,
}

impl<'a> Route<'a> {
    pub fn create(departure: &'a Airport, arrival: &'a Airport, mach: f32) -> Self {
        let mut route = Route {
            departure: &departure,
            arrival:   &arrival,
            distance:  0.0,
            time:      0.0,
        };

        route.distance = route.calculate_distance();
        route.time     = route.calculate_time(mach);
        route
    }

    pub fn from_icao(dep_icao: &str, arr_icao: &str, mach: f32, airports: &'a [Airport]) ->
        Result<Route<'a>> {

        let dep = airports.find_by_icao(dep_icao).ok_or(
            "departure not found")?;

        let arr = airports.find_by_icao(arr_icao).ok_or(
            "arrival not found")?;

        Ok(Route::create(&dep, &arr, mach))
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

    pub fn is_filter_match(&self, filters: &[RouteFilter]) -> bool {
        let matches = filters.iter().all(|ref filter| {
            use self::RouteFilter::*;
            
            match **filter {
                ArrType(ref _type)           => self.arrival._type == *_type,
                ArrRunwayLength(ref len)     => len.any_match(&self.arrival.runways),
                ArrCountries(ref countries)  => countries.any_match(&self.arrival.region.code),
                MinTime(Time(min_time))      => self.time >= min_time,
                MaxTime(Time(max_time))      => self.time <= max_time,
            }
        });

        matches && self.arrival.icao != self.departure.icao
    }

    pub fn get_all_filter_matches(form: &DataForm, airports: &'a [Airport])
        -> Result<Vec<Route<'a>>> {

        match (&form.dep_icao, &form.arr_icao) {
            (&Some(ref dep_icao), &Some(ref arr_icao)) => {
                Route::from_icao(
                    dep_icao,
                    arr_icao,
                    form.mach,
                    airports
                ).map(|route| vec![route])
            },
            _ => Route::filter_routes(&form, &airports),
        }
    }

    fn filter_routes(form: &DataForm, airports: &'a [Airport]) ->
        Result<Vec<Route<'a>>> {

        let departure = airports.find_by_form(&form).ok_or(
            "departure airport not found")?;

        let filters = RouteFilter::from_form(&form);

        let mut routes = airports.par_iter()
            .filter_map(|arrival| {
                let route = Route::create(&departure,
                                          &arrival,
                                          form.mach);

                if route.is_filter_match(&filters) {
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