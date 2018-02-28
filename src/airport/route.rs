extern crate rayon;

use airport::{self, Airport, Countries, RunwayLength};
use self::rayon::prelude::*;

error_chain!{}

type Hours = f32;

#[derive(Debug)]
pub enum RouteFilter {
    ArrType(airport::Type),
    ArrRunwayLength(RunwayLength),
    ArrCountries(Countries),
    MinTime(Hours),
    MaxTime(Hours),
}

#[derive(Debug, Serialize)]
pub struct Route<'a> {
    pub departure: &'a Airport,
    pub arrival: &'a Airport,
    distance: Option<f32>,
    time: Option<f32>,
}

impl<'a> Route<'a> {
    pub fn create(departure: &'a Airport, arrival: &'a Airport) -> Self {
        Route {
            departure,
            arrival,
            distance: None,
            time: None,
        }
    }

    pub fn from_icao(dep_icao: &str, arr_icao: &str, airports: &'a [Airport]) -> Result<Route<'a>> {
        let dep = airport::find_by_icao(dep_icao, airports).ok_or("departure not found")?;
        let arr = airport::find_by_icao(arr_icao, airports).ok_or("arrival not found")?;

        Ok(Route::create(dep, arr))
    }

    fn calculate_distance(&self) -> f32 {
        let radius = 3440.; // Earth's radius in nautical miles

        let lat1 = self.departure.pos.lat.to_radians();
        let lat2 = self.arrival.pos.lat.to_radians();
        let lat = lat2 - lat1;
        let lon = (self.arrival.pos.lon - self.departure.pos.lon).to_radians();

        let a = (lat / 2.0).sin().powi(2) + lat1.cos() * lat2.cos() * (lon / 2.0).sin().powi(2);
        let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

        radius * c
    }

    fn calculate_time(&mut self, cruise_speed: &Speed) -> f32 {
        self.distance() / cruise_speed.as_knots()
    }

    pub fn distance(&mut self) -> f32 {
        self.distance.unwrap_or_else(|| {
            let distance = self.calculate_distance();
            self.distance = Some(distance);
            distance
        })
    }

    pub fn time(&mut self, cruise_speed: &Speed) -> f32 {
        self.time.unwrap_or_else(|| {
            let time = self.calculate_time(cruise_speed);
            self.time = Some(time);
            time
        })
    }

    pub fn eval_lazy(&mut self, cruise_speed: &Speed) {
        // Evaluating the route time will force the evaluation of the distance as well.
        // This may also look a bit hacky, but it reduces the amount of code duplication
        self.time(cruise_speed);
    }

    pub fn passes_filters(&mut self, filters: &[RouteFilter], cruise_speed: &Speed) -> bool {
        let matches = filters.iter().all(|filter| {
            use self::RouteFilter::*;

            match *filter {
                ArrType(ref _type) => self.arrival._type == *_type,
                ArrRunwayLength(ref len) => len.any_match(&self.arrival.runways),
                ArrCountries(ref countries) => countries.any_match(&self.arrival.region.code),
                MinTime(min_time) => self.time(cruise_speed) >= min_time,
                MaxTime(max_time) => self.time(cruise_speed) <= max_time,
            }
        });

        matches && self.arrival.icao != self.departure.icao
    }
}

#[derive(Debug)]
pub enum Speed {
    Mach(f32),
    Knots(f32),
}

impl Speed {
    const KNOTS_PER_MACH: f32 = 666.739;

    pub fn as_knots(&self) -> f32 {
        match *self {
            Speed::Mach(mach) => mach * Speed::KNOTS_PER_MACH,
            Speed::Knots(knots) => knots,
        }
    }
}

#[derive(Debug)]
pub enum SortBy {
    Distance,
    ArrICAO,
}

pub fn find_all<'a>(
    departure: &'a Airport,
    route_filters: &[RouteFilter],
    cruise_speed: &Speed,
    sorter: &SortBy,
    airports: &'a [Airport],
) -> Result<Vec<Route<'a>>> {
    let mut routes = airports
        .par_iter()
        .filter_map(|arrival| {
            let mut route = Route::create(departure, arrival);

            if route.passes_filters(route_filters, cruise_speed) {
                route.eval_lazy(cruise_speed);
                Some(route)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    match *sorter {
        // Every lazy value has been evaluated at this point, so it's safe to unwrap
        SortBy::Distance => routes.sort_by_key(|route| route.distance.unwrap() as i32),
        SortBy::ArrICAO => routes.sort_by_key(|route| &route.arrival.icao),
    }

    routes.truncate(100);
    Ok(routes)
}
