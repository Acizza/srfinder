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

        let type_filter = match self.airport_type {
            AirportType::Unknown => OptionalFilter::Passthrough,
            kind => OptionalFilter::Evaluate(move |arpt| arpt.class == kind),
        };

        let runway_len_filter = match self.runway_length {
            Some(len) => OptionalFilter::Evaluate(move |arpt| len.fits_any(&arpt.runways)),
            None => OptionalFilter::Passthrough,
        };

        let country_filter = match self.countries.as_slice() {
            [] => OptionalFilter::Passthrough,
            countries => OptionalFilter::Evaluate(move |arpt| {
                Self::list_has_any(&arpt.country_name, countries)
            }),
        };

        Self::airport_matches(
            |arpt| {
                type_filter.eval(arpt) && runway_len_filter.eval(arpt) && country_filter.eval(arpt)
            },
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

enum OptionalFilter<F>
where
    F: Fn(&Airport) -> bool,
{
    Evaluate(F),
    Passthrough,
}

impl<F> OptionalFilter<F>
where
    F: Fn(&Airport) -> bool,
{
    fn eval(&self, airport: &Airport) -> bool {
        match self {
            Self::Evaluate(func) => func(airport),
            Self::Passthrough => true,
        }
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
    use crate::airport_data::RunwayMarker;
    use std::collections::HashMap;

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

    fn get_airports() -> Vec<Airport> {
        // San Francisco
        let ksfo = Airport {
            icao: "KSFO".into(),
            class: AirportType::Large,
            position: Position::new(37.618, -122.375),
            runways: vec![Runway {
                length_ft: Some(7500),
                width_ft: Some(200),
                he_marker: Some(RunwayMarker::new("01L".into(), 0.0, 0.0)),
                le_marker: Some(RunwayMarker::new("19R".into(), 0.0, 0.0)),
            }],
            frequencies: HashMap::new(),
            country_name: "United States".into(),
        };

        // Sacramento Executive
        let ksac = Airport {
            icao: "KSAC".into(),
            class: AirportType::Medium,
            position: Position::new(38.512, -121.492),
            runways: vec![Runway {
                length_ft: Some(3836),
                width_ft: Some(100),
                he_marker: Some(RunwayMarker::new("12".into(), 0.0, 0.0)),
                le_marker: Some(RunwayMarker::new("30".into(), 0.0, 0.0)),
            }],
            frequencies: HashMap::new(),
            country_name: "United States".into(),
        };

        // Narita
        let rjaa = Airport {
            icao: "RJAA".into(),
            class: AirportType::Large,
            position: Position::new(35.764, 140.386),
            runways: vec![Runway {
                length_ft: Some(8202),
                width_ft: Some(196),
                he_marker: Some(RunwayMarker::new("16L".into(), 0.0, 0.0)),
                le_marker: Some(RunwayMarker::new("34R".into(), 0.0, 0.0)),
            }],
            frequencies: HashMap::new(),
            country_name: "Japan".into(),
        };

        vec![ksfo, ksac, rjaa]
    }

    fn display_airports(airports: Vec<&Airport>) -> Vec<&String> {
        airports.into_iter().map(|arpt| &arpt.icao).collect()
    }

    macro_rules! assert_expected_icaos {
        ($result:expr, $expected:expr) => {
            let result = $result;

            assert!(
                !result.is_empty(),
                "expected to get airports {:?}, got nothing",
                $expected
            );

            assert_eq!(
                $expected.len(),
                result.len(),
                "expected to get {} airport(s) ({:?}), got {} ({:?})",
                $expected.len(),
                $expected,
                result.len(),
                display_airports(result)
            );

            let has_expected_arpts = result
                .iter()
                .all(|found| $expected.contains(&found.icao.as_str()));

            assert!(
                has_expected_arpts,
                "expected to get airports {:?}, got {:?}",
                $expected,
                display_airports(result)
            );
        };
    }

    #[test]
    fn filter_icao() {
        const EXPECTED_ICAO: &'static str = "KSAC";

        let airports = get_airports();

        let filter = AirportFilters {
            icao: Some(EXPECTED_ICAO.into()),
            airport_type: AirportType::Unknown,
            runway_length: None,
            countries: vec![],
        };

        assert_expected_icaos!(filter.matching_airports(&airports[..]), [EXPECTED_ICAO]);
    }

    #[test]
    fn filter_airport_type() {
        const EXPECTED_TYPE: AirportType = AirportType::Large;
        const EXPECTED_ICAOS: [&'static str; 2] = ["KSFO", "RJAA"];

        let airports = get_airports();

        let filter = AirportFilters {
            icao: None,
            airport_type: EXPECTED_TYPE,
            runway_length: None,
            countries: vec![],
        };

        assert_expected_icaos!(filter.matching_airports(&airports[..]), EXPECTED_ICAOS);
    }

    #[test]
    fn filter_runway_length_gt() {
        const EXPECTED_ICAOS: [&'static str; 2] = ["KSFO", "RJAA"];

        let airports = get_airports();

        let filter = AirportFilters {
            icao: None,
            airport_type: AirportType::Unknown,
            runway_length: Some(RunwayLength::GreaterThan(7000)),
            countries: vec![],
        };

        assert_expected_icaos!(filter.matching_airports(&airports[..]), EXPECTED_ICAOS);
    }

    #[test]
    fn filter_runway_length_eq() {
        const EXPECTED_ICAOS: [&'static str; 1] = ["KSAC"];

        let airports = get_airports();

        let filter = AirportFilters {
            icao: None,
            airport_type: AirportType::Unknown,
            runway_length: Some(RunwayLength::Equal(3836)),
            countries: vec![],
        };

        assert_expected_icaos!(filter.matching_airports(&airports[..]), EXPECTED_ICAOS);
    }

    #[test]
    fn filter_runway_length_lt() {
        const EXPECTED_ICAOS: [&'static str; 2] = ["KSFO", "KSAC"];

        let airports = get_airports();

        let filter = AirportFilters {
            icao: None,
            airport_type: AirportType::Unknown,
            runway_length: Some(RunwayLength::LessThan(7501)),
            countries: vec![],
        };

        assert_expected_icaos!(filter.matching_airports(&airports[..]), EXPECTED_ICAOS);
    }

    #[test]
    fn filter_countries() {
        let airports = get_airports();

        let mut filter = AirportFilters {
            icao: None,
            airport_type: AirportType::Unknown,
            runway_length: None,
            countries: vec!["United States".into()],
        };

        assert_expected_icaos!(filter.matching_airports(&airports[..]), ["KSFO", "KSAC"]);

        filter.countries = vec!["Japan".into()];
        assert_expected_icaos!(filter.matching_airports(&airports[..]), ["RJAA"]);

        filter.countries = vec!["United States".into(), "Japan".into()];

        assert_expected_icaos!(
            filter.matching_airports(&airports[..]),
            ["KSFO", "KSAC", "RJAA"]
        );
    }
}
