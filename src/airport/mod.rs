extern crate rand;

pub mod route;
pub mod data;

use self::rand::Rng;
use std::ops::Deref;

#[derive(Debug)]
pub enum AirportFilter {
    Type(Type),
    RunwayLength(RunwayLength),
    Countries(Countries),
}

#[derive(Debug, Serialize)]
pub struct Airport {
    pub icao:        String,
    pub name:        String,
    pub pos:         LatLon,
    pub _type:       Type,
    pub runways:     Option<Vec<Runway>>,
    pub frequencies: Option<Frequencies>,
    pub region:      Region,
}

impl Airport {
    pub fn passes_filters(&self, filters: &[AirportFilter]) -> bool {
        filters.iter().all(|ref filter| {
            use self::AirportFilter::*;

            match **filter {
                Type(ref _type)          => self._type == *_type,
                RunwayLength(ref len)    => len.any_match(&self.runways),
                Countries(ref countries) => countries.any_match(&self.region.code),
            }
        })
    }
}

pub fn find_by_icao<'a>(icao: &str, airports: &'a [Airport]) -> Option<&'a Airport> {
    airports.iter().find(|arpt| arpt.icao == icao)
}

pub fn find<'a>(filters: &[AirportFilter], airports: &'a [Airport]) -> Option<&'a Airport> {
    let found = airports.iter()
        .filter(|&airport| airport.passes_filters(filters))
        .collect::<Vec<_>>();

    if found.len() > 0 {
        Some(found[rand::thread_rng().gen_range(0, found.len())])
    } else {
        None
    }
}

#[derive(Debug, Serialize)]
pub struct LatLon {
    pub lat: f32,
    pub lon: f32,
}

#[derive(Debug, Serialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Type {
    Closed,
    SeaplaneBase,
    Heliport,
    Small,
    Medium,
    Large,
}

impl Type {
    pub fn from_str(value: &str) -> Option<Type> {
        match value {
            "closed"         => Some(Type::Closed),
            "seaplane_base"  => Some(Type::SeaplaneBase),
            "heliport"       => Some(Type::Heliport),
            "small_airport"  => Some(Type::Small),
            "medium_airport" => Some(Type::Medium),
            "large_airport"  => Some(Type::Large),
            _                => None,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Runway {
    pub sides:  RunwaySides,
    pub width:  Option<u32>,
    pub length: Option<u32>,
}

#[derive(Debug, Serialize)]
pub struct RunwaySideData {
    pub name: String,
    pub pos:  Option<LatLon>,
}

#[derive(Debug, Serialize)]
pub struct RunwaySides {
    pub north: RunwaySideData,
    pub south: RunwaySideData,
}

#[derive(Debug, Serialize)]
pub struct Frequencies {
    pub ground:    Option<String>,
    pub tower:     Option<String>,
    pub departure: Option<String>,
    pub approach:  Option<String>,
    pub atis:      Option<String>,
}

#[derive(Debug, Serialize)]
pub struct Region {
    pub code:      String,
    pub continent: String,
}

type Length    = u32;
type MinLength = Length;
type MaxLength = Length;

#[derive(Debug, Clone)]
pub enum RunwayLength {
    LessThan(Length),
    GreaterThan(Length),
    EqualTo(Length),
    Between(MinLength, MaxLength),
}

impl RunwayLength {
    fn parse_single(input_str: &str) -> Option<RunwayLength> {
        let mut chars = input_str.chars();
        
        let order = try_opt!(chars.next());
        let value = try_opt!(chars.collect::<String>().parse().ok());

        use self::RunwayLength::*;

        match order {
            '<' => Some(LessThan(value)),
            '>' => Some(GreaterThan(value)),
            '=' => Some(EqualTo(value)),
            _   => None,
        }
    }

    fn parse_between(input_str: &str) -> Option<RunwayLength> {
        let values = input_str.splitn(2, '+')
                              .collect::<Vec<_>>();

        if values.len() == 2 {
            let min = try_opt!(values[0].parse().ok());
            let max = try_opt!(values[1].parse().ok());

            Some(RunwayLength::Between(min, max))
        } else {
            None
        }
    }

    pub fn parse(input_str: &str) -> Option<RunwayLength> {
        RunwayLength::parse_between(input_str)
            .or(RunwayLength::parse_single(input_str))
    }

    pub fn is_match(&self, runway: &Runway) -> bool {
        match runway.length {
            Some(run_len) => {
                use self::RunwayLength::*;
                match *self {
                    LessThan(len)             => run_len <= len,
                    GreaterThan(len)          => run_len >= len,
                    EqualTo(len)              => run_len == len,
                    Between(min_len, max_len) => run_len >= min_len && run_len <= max_len,
                }
            },
            None => false,
        }
    }

    pub fn any_match(&self, runways: &Option<Vec<Runway>>) -> bool {
        match *runways {
            Some(ref runways) => runways.iter().any(|r| self.is_match(r)),
            None => false,
        }
    }
}

type CountryCode = String;

#[derive(Debug, Clone)]
pub struct Countries(pub Vec<CountryCode>);

impl Countries {
    pub fn any_match(&self, code: &str) -> bool {
        self.is_empty() || self.iter().any(|c| c == code)
    }
}

impl Deref for Countries {
    type Target = Vec<CountryCode>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}