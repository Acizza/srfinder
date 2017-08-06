extern crate rand;

use filter::{DataForm, AirportFilter};
use ::rocket::http::RawStr;
use ::rocket::request::FromFormValue;
use self::rand::Rng;
use std::ascii::AsciiExt;

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

#[derive(Debug, Serialize)]
pub struct Runway {
    pub sides:   RunwaySides,
    pub width:  Option<u32>,
    pub length: Option<u32>,
    pub closed: Option<bool>,
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

impl Type {
    pub fn parse(value: &str) -> Option<Type> {
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

impl<'v> FromFormValue<'v> for Type {
    type Error = &'v RawStr;

    fn from_form_value(form_value: &'v RawStr) -> Result<Type, &'v RawStr> {
        let value = form_value.as_str().to_ascii_lowercase();

        match value.as_str() {
            "closed"       => Ok(Type::Closed),
            "seaplanebase" => Ok(Type::SeaplaneBase),
            "heliport"     => Ok(Type::Heliport),
            "small"        => Ok(Type::Small),
            "medium"       => Ok(Type::Medium),
            "large"        => Ok(Type::Large),
            _              => Err(form_value),
        }
    }
}

pub trait AirportSearching<'a> {
    fn find_by_icao(&self, icao: &str) -> Option<&'a Airport>;
    fn find_by_form(&self, form: &DataForm) -> Option<&'a Airport>;
    fn find_by_filters(&self, filters: &[AirportFilter]) -> Vec<&'a Airport>;
}

impl<'a> AirportSearching<'a> for &'a [Airport] {
    fn find_by_icao(&self, icao: &str) -> Option<&'a Airport> {
        self.iter().find(|&airport| airport.icao == icao)
    }

    fn find_by_form(&self, form: &DataForm) -> Option<&'a Airport> {
        match form.dep_icao {
            Some(ref icao) => self.find_by_icao(icao),
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
                    Type(ref _type)           => airport._type == *_type,
                    RunwayLength(ref len)     => len.any_match(&airport.runways),
                    Countries(ref countries)  => countries.any_match(&airport.region.code),
                }
            })
        })
        .collect()
    }
}