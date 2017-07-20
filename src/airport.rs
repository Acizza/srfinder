use std::ascii::AsciiExt;
use ::rocket::http::RawStr;
use ::rocket::request::FromFormValue;

#[derive(Debug, Serialize)]
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

#[derive(Debug)]
pub enum Size {
    Heliport,
    Small,
    Medium,
    Large
}

impl<'v> FromFormValue<'v> for Size {
    type Error = &'v RawStr;

    fn from_form_value(form_value: &'v RawStr) -> Result<Size, &'v RawStr> {
        let value = form_value.as_str().to_ascii_lowercase();

        match value.as_str() {
            "heliport" => Ok(Size::Heliport),
            "small"    => Ok(Size::Small),
            "medium"   => Ok(Size::Medium),
            "large"    => Ok(Size::Large),
            _          => Err(form_value),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct LatLon {
    pub lat: f64,
    pub lon: f64,
}

impl LatLon {
    pub fn new(lat: f64, lon: f64) -> LatLon {
        LatLon { lat, lon }
    }
}

#[derive(Debug, Serialize)]
pub struct Airport {
    pub pos:  LatLon,
    pub icao: ICAO,
}