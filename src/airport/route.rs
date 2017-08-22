extern crate rayon;

use airport::{self, Airport, RunwayLength, Countries};
use self::rayon::prelude::*;

error_chain! {}

type Hours = f32;

#[derive(Debug)]
pub enum RouteFilter {
    ArrType(airport::Type),
    ArrRunwayLength(RunwayLength),
    ArrCountries(Countries),
    MinTime(Hours),
    MaxTime(Hours),
}

const KNOTS_PER_MACH: f32 = 666.739;

#[derive(Debug, Serialize)]
pub struct Route<'a> {
    pub departure: &'a Airport,
    pub arrival:   &'a Airport,
    distance:      Option<f32>,
    time:          Option<f32>,
}

impl<'a> Route<'a> {
    pub fn create(departure: &'a Airport, arrival: &'a Airport) -> Self {
        Route {
            departure: &departure,
            arrival:   &arrival,
            distance:  None,
            time:      None,
        }
    }

    pub fn from_icao(dep_icao: &str, arr_icao: &str, airports: &'a [Airport]) ->
        Result<Route<'a>> {

        let dep = airport::find_by_icao(dep_icao, airports).ok_or(
            "departure not found")?;

        let arr = airport::find_by_icao(arr_icao, airports).ok_or(
            "arrival not found")?;

        Ok(Route::create(&dep, &arr))
    }

    fn calculate_distance(&self) -> f32 {
        let radius = 3440.; // Earth's radius in nautical miles

        let lat1 = self.departure.pos.lat.to_radians();
        let lat2 = self.arrival.pos.lat.to_radians();
        let lat  = lat2 - lat1;
        let lon  = (self.arrival.pos.lon - self.departure.pos.lon).to_radians();

        let a = (lat / 2.0).sin().powi(2) + lat1.cos() * lat2.cos() * (lon / 2.0).sin().powi(2);
        let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

        radius * c
    }

    fn calculate_time(&mut self, mach: f32) -> f32 {
        self.distance() / (mach * KNOTS_PER_MACH)
    }

    pub fn distance(&mut self) -> f32 {
        self.distance.unwrap_or_else(|| {
            let distance = self.calculate_distance();
            self.distance = Some(distance);
            distance
        })
    }

    pub fn time(&mut self, mach: f32) -> f32 {
        self.time.unwrap_or_else(|| {
            let time = self.calculate_time(mach);
            self.time = Some(time);
            time
        })
    }

    pub fn eval_lazy(&mut self, mach: f32) {
        // Evaluating the route time will force the evaluation of the distance as well.
        // This may also look a bit hacky, but it reduces the amount of code duplication
        self.time(mach);
    }

    pub fn passes_filters(&mut self, filters: &[RouteFilter], mach: f32) -> bool {
        let matches = filters.iter().all(|ref filter| {
            use self::RouteFilter::*;
            
            match **filter {
                ArrType(ref _type)          => self.arrival._type == *_type,
                ArrRunwayLength(ref len)    => len.any_match(&self.arrival.runways),
                ArrCountries(ref countries) => countries.any_match(&self.arrival.region.code),
                MinTime(min_time)           => self.time(mach) >= min_time,
                MaxTime(max_time)           => self.time(mach) <= max_time,
            }
        });

        matches && self.arrival.icao != self.departure.icao
    }
}

pub fn find_all<'a>(
    departure: &'a Airport,
    route_filters: &[RouteFilter],
    mach: f32,
    airports: &'a [Airport]) -> Result<Vec<Route<'a>>> {

    let mut routes = airports.par_iter()
        .filter_map(|arrival| {
            let mut route = Route::create(&departure,
                                          &arrival);

            if route.passes_filters(route_filters, mach) {
                route.eval_lazy(mach);
                Some(route)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    // Every lazy value has been evaluated at this point, so it's safe to unwrap
    routes.sort_by_key(|route| route.distance.unwrap() as i32);

    Ok(routes.into_iter()
            .take(100)
            .collect())
}