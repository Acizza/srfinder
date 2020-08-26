use crate::airport_data::{Airport, AirportType, Position, Runway};
use rand::seq::SliceRandom;
use rocket::State;
use rocket_contrib::json::{Json, JsonValue};
use serde_derive::{Deserialize, Serialize};
use smallvec::{smallvec, SmallVec};
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
fn random_airport_set(airports: &[Airport]) -> AirportList {
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
pub struct Time {
    pub hour: u8,
    pub minutes: u8,
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
    pub speed: Speed,
    pub departure: Option<AirportFilters>,
    pub arrival: Option<AirportFilters>,
    #[serde(rename = "timeDist", default)]
    pub time_or_dist: Option<TimeOrDistance>,
}

type AirportList<'a> = SmallVec<[&'a Airport; 1]>;

#[derive(Debug, Deserialize)]
pub struct AirportFilters {
    pub icao: Option<String>,
    #[serde(rename = "airportType", default)]
    pub airport_type: AirportType,
    #[serde(rename = "runwayLength")]
    pub runway_length: Option<RunwayLength>,
    #[serde(default)]
    pub countries: Vec<String>,
}

impl AirportFilters {
    pub fn matching_airports<'a>(&self, airports: &'a [Airport]) -> AirportList<'a> {
        if let Some(icao) = &self.icao {
            let index = airports
                .binary_search_by(|arpt| arpt.icao.as_str().cmp(icao.as_str()))
                .ok();

            let index = match index {
                Some(index) => index,
                None => return SmallVec::new(),
            };

            return smallvec![&airports[index]];
        }

        // TODO: convert to macro
        match (
            self.airport_type,
            &self.runway_length,
            self.countries.as_slice(),
        ) {
            (AirportType::Unknown, None, []) => Self::airport_matches(|_| true, airports),
            (arpt_type, None, []) => {
                Self::airport_matches(|arpt| arpt.class == arpt_type, airports)
            }
            (AirportType::Unknown, Some(len), []) => {
                Self::airport_matches(|arpt| len.fits_any(&arpt.runways), airports)
            }
            (arpt_type, Some(len), []) => Self::airport_matches(
                |arpt| arpt.class == arpt_type && len.fits_any(&arpt.runways),
                airports,
            ),
            (AirportType::Unknown, Some(len), countries) => Self::airport_matches(
                |arpt| {
                    len.fits_any(&arpt.runways) && Self::list_has_any(&arpt.country_name, countries)
                },
                airports,
            ),
            (arpt_type, Some(len), countries) => Self::airport_matches(
                |arpt| {
                    arpt.class == arpt_type
                        && len.fits_any(&arpt.runways)
                        && Self::list_has_any(&arpt.country_name, countries)
                },
                airports,
            ),
            (AirportType::Unknown, None, countries) => Self::airport_matches(
                |arpt| Self::list_has_any(&arpt.country_name, countries),
                airports,
            ),
            (arpt_type, None, countries) => Self::airport_matches(
                |arpt| arpt.class == arpt_type && Self::list_has_any(&arpt.country_name, countries),
                airports,
            ),
        }
    }

    fn airport_matches<F>(matcher: F, airports: &[Airport]) -> AirportList
    where
        F: Fn(&Airport) -> bool,
    {
        let mut results = SmallVec::new();

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
        if let Some(min) = &self.min {
            if value < min {
                return false;
            }
        }

        if let Some(max) = &self.max {
            if value > max {
                return false;
            }
        }

        true
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
#[serde(tag = "type", content = "value")]
pub enum TimeOrDistance {
    #[serde(rename = "time")]
    Time(Range<Time>),
    #[serde(rename = "dist")]
    Distance(Range<f32>),
}
