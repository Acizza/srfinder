use crate::airport_data::{Airport, AirportType, Position, Runway};
use rand::seq::SliceRandom;
use rocket::State;
use rocket_contrib::json::{Json, JsonValue};
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::ptr;

const MAX_AIRPORTS_TO_GET: usize = 2000;
const MAX_AIRPORTS_TO_RETURN: usize = 100;

#[post("/search_routes", format = "json", data = "<filters>")]
pub fn search_routes<'a>(filters: Json<Filters>, airports: State<'a, Vec<Airport>>) -> JsonValue {
    let departures = filters
        .departure
        .as_ref()
        .map(|filters| filters.matching_airports(&airports))
        .unwrap_or_else(|| random_airport_set(&airports));

    let arrivals = filters
        .arrival
        .as_ref()
        .map(|filters| filters.matching_airports(&airports))
        .unwrap_or_else(|| random_airport_set(&airports));

    let mut routes = Vec::with_capacity(MAX_AIRPORTS_TO_RETURN / 2);

    for departure in departures {
        for arrival in &arrivals {
            if ptr::eq(departure, *arrival) {
                continue;
            }

            let route = Route::new(departure, arrival, filters.speed);

            match &filters.time_or_dist {
                Some(TimeOrDistance::Time(time_range)) => {
                    if !time_range.within(&route.time) {
                        continue;
                    }
                }
                Some(TimeOrDistance::Distance(dist_range)) => {
                    if !dist_range.within(&route.distance) {
                        continue;
                    }
                }
                None => (),
            }

            routes.push(route);
        }
    }

    let mut routes = routes.as_mut_slice();

    if routes.len() > 1 {
        let (shuffled, _) = routes.partial_shuffle(&mut rand::thread_rng(), MAX_AIRPORTS_TO_RETURN);
        routes = shuffled;
    }

    json!({ "routes": routes })
}

#[inline(always)]
fn random_airport_set(airports: &[Airport]) -> Vec<&Airport> {
    airports
        .choose_multiple(&mut rand::thread_rng(), MAX_AIRPORTS_TO_GET)
        .collect()
}

#[derive(Debug, Serialize)]
struct Route<'a> {
    from: &'a Airport,
    to: &'a Airport,
    distance: f32,
    time: Time,
}

impl<'a> Route<'a> {
    #[inline(always)]
    fn new(from: &'a Airport, to: &'a Airport, speed: Speed) -> Self {
        let distance = Self::calculate_distance(from.position, to.position);
        let time = Time::from_distance(distance, speed);

        Self {
            from,
            to,
            distance,
            time,
        }
    }

    /// Get the distance in nautical miles between two longitude and latitude points
    /// using the Haversine formula.
    fn calculate_distance(from: Position, to: Position) -> f32 {
        // Radius of Earth in nautical miles
        const RADIUS_NM: f32 = 3440.0;

        let from_lat = from.latitude_deg.to_radians();
        let to_lat = to.latitude_deg.to_radians();
        let lat = to_lat - from_lat;

        let lon = (to.longitude_deg - from.longitude_deg).to_radians();

        let a =
            (lat / 2.0).sin().powi(2) + from_lat.cos() * to_lat.cos() * (lon / 2.0).sin().powi(2);
        let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

        RADIUS_NM * c
    }
}

#[derive(Copy, Clone, Debug, Default, Deserialize, Serialize, PartialEq, PartialOrd)]
struct Time {
    hour: u8,
    minutes: u8,
}

impl Time {
    fn from_distance(distance: f32, speed: Speed) -> Self {
        let total_hours = distance / speed.as_knots();

        let hour = total_hours.floor().min(99.0);
        let minutes = ((total_hours - hour) * 60.0).floor().min(59.0);

        Self {
            hour: hour as u8,
            minutes: minutes as u8,
        }
    }
}

#[derive(Copy, Clone, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
#[serde(tag = "type", content = "value")]
pub enum Speed {
    Mach(f32),
    Knots(f32),
}

impl Speed {
    #[inline(always)]
    fn as_knots(self) -> f32 {
        const KNOTS_PER_MACH: f32 = 666.739;

        match self {
            Self::Mach(mach) => mach * KNOTS_PER_MACH,
            Self::Knots(knots) => knots,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Filters {
    speed: Speed,
    departure: Option<AirportFilters>,
    arrival: Option<AirportFilters>,
    #[serde(rename = "timeDist", default)]
    time_or_dist: Option<TimeOrDistance>,
}

#[derive(Debug, Deserialize)]
struct AirportFilters {
    icao: Option<String>,
    #[serde(rename = "airportType", default)]
    airport_type: AirportType,
    #[serde(rename = "runwayLength")]
    runway_length: Option<RunwayLength>,
    #[serde(default)]
    countries: Vec<String>,
}

impl AirportFilters {
    fn matching_airports<'a>(&self, airports: &'a [Airport]) -> Vec<&'a Airport> {
        if let Some(icao) = &self.icao {
            let index = airports
                .binary_search_by(|arpt| arpt.icao.as_str().cmp(icao.as_str()))
                .ok();

            let index = match index {
                Some(index) => index,
                None => return Vec::new(),
            };

            return vec![&airports[index]];
        }

        type MatchClosure<'a> = Box<dyn Fn(&Airport) -> bool + 'a>;

        let type_matches: MatchClosure = match self.airport_type {
            AirportType::Unknown => Box::new(|_| true),
            kind => Box::new(move |arpt| arpt.class == kind),
        };

        let runway_len_matches: MatchClosure = match self.runway_length {
            Some(len) => Box::new(move |arpt| len.fits_any(&arpt.runways)),
            None => Box::new(|_| true),
        };

        let country_matches: MatchClosure = match self.countries.as_slice() {
            [] => Box::new(|_| true),
            countries => Box::new(move |arpt| Self::list_has_any(&arpt.country_name, countries)),
        };

        Self::airport_matches(
            |arpt| type_matches(arpt) && runway_len_matches(arpt) && country_matches(arpt),
            airports,
        )
    }

    fn airport_matches<F>(matcher: F, airports: &[Airport]) -> Vec<&Airport>
    where
        F: Fn(&Airport) -> bool,
    {
        let mut results = Vec::new();

        for airport in airports {
            if !matcher(airport) {
                continue;
            }

            results.push(airport);

            if results.len() >= MAX_AIRPORTS_TO_GET {
                break;
            }
        }

        results
    }

    #[inline(always)]
    fn list_has_any(name: &str, list: &[String]) -> bool {
        list.iter().any(|x| x.eq_ignore_ascii_case(name))
    }
}

#[derive(Copy, Clone, Debug, Deserialize)]
#[serde(tag = "selector", content = "length")]
pub enum RunwayLength {
    #[serde(rename = "eq")]
    Equal(u32),
    #[serde(rename = "gt")]
    GreaterThan(u32),
    #[serde(rename = "lt")]
    LessThan(u32),
}

impl RunwayLength {
    #[inline(always)]
    pub fn fits(self, length: u32) -> bool {
        match self {
            Self::Equal(len) => len == length,
            Self::GreaterThan(len) => length > len,
            Self::LessThan(len) => length < len,
        }
    }

    #[inline(always)]
    pub fn fits_any(self, runways: &[Runway]) -> bool {
        runways.iter().any(|runway| match runway.length_ft {
            Some(len) => self.fits(len),
            None => false,
        })
    }
}

#[derive(Debug, Default, Deserialize)]
pub struct Range<T>
where
    T: fmt::Debug + Default + PartialOrd,
{
    pub min: Option<T>,
    pub max: Option<T>,
}

impl<T> Range<T>
where
    T: fmt::Debug + Default + PartialOrd,
{
    fn within(&self, value: &T) -> bool {
        if let Some(true) = self.min.as_ref().map(|min| value < min) {
            return false;
        }

        if let Some(true) = self.max.as_ref().map(|max| value > max) {
            return false;
        }

        true
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
#[serde(tag = "type", content = "value")]
enum TimeOrDistance {
    #[serde(rename = "time")]
    Time(Range<Time>),
    #[serde(rename = "dist")]
    Distance(Range<f32>),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn range_within() {
        macro_rules! within {
            ($min:expr => $max:expr, $value:expr) => {
                Range::within(
                    &Range {
                        min: $min,
                        max: $max,
                    },
                    &$value,
                )
            };
        }

        assert_eq!(within!(Some(1) => Some(3), 2), true);
        assert_eq!(within!(Some(2) => Some(3), 2), true);
        assert_eq!(within!(Some(2) => Some(3), 3), true);

        assert_eq!(within!(Some(1) => Some(5), 10), false);
        assert_eq!(within!(Some(10) => Some(100), 9), false);
    }

    #[test]
    fn runway_length_fits() {
        assert_eq!(RunwayLength::Equal(12345).fits(12345), true);
        assert_eq!(RunwayLength::Equal(12345).fits(12346), false);

        assert_eq!(RunwayLength::GreaterThan(1000).fits(1001), true);
        assert_eq!(RunwayLength::GreaterThan(1000).fits(1000), false);

        assert_eq!(RunwayLength::LessThan(1000).fits(999), true);
        assert_eq!(RunwayLength::LessThan(1000).fits(1000), false);
    }
}
