pub mod data;

use std::ascii::AsciiExt;
use ::rocket::http::RawStr;
use ::rocket::request::FromFormValue;

#[derive(Debug, Serialize, Clone)]
pub struct ICAO(pub String);

impl<'v> FromFormValue<'v> for ICAO {
    type Error = &'v RawStr;

    fn from_form_value(form_value: &'v RawStr) -> Result<ICAO, &'v RawStr> {
        if form_value != "" {
            Ok(ICAO(form_value.as_str().into()))
        } else {
            Err(form_value)
        }
    }
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

#[derive(Debug, Serialize, Clone)]
pub struct LatLon {
    pub lat: f32,
    pub lon: f32,
}

#[derive(Debug, Serialize, Clone)]
pub struct RunwayIdentifier {
    pub north: String,
    pub south: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct Runway {
    pub ident:  RunwayIdentifier,
    pub width:  Option<u32>,
    pub length: Option<u32>,
    pub closed: Option<bool>,
}

#[derive(Debug, Serialize, Clone)]
pub struct Frequencies {
    pub ground:    Option<String>,
    pub tower:     Option<String>,
    pub departure: Option<String>,
    pub approach:  Option<String>,
    pub atis:      Option<String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct Airport {
    pub icao:        String,
    pub pos:         LatLon,
    pub _type:       Type,
    pub runways:     Option<Vec<Runway>>,
    pub frequencies: Option<Frequencies>,
}