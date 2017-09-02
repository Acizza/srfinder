use airport::{self, AirportFilter, RunwayLength, Countries, Type};
use airport::route::{self, RouteFilter};
use rocket::http::RawStr;
use rocket::request::FromFormValue;
use std::ascii::AsciiExt;
use std::ops::Deref;
use util::ToEnum;

macro_rules! gen_val_type {
    (self, $value:ident)  => ($value.clone());
    (inner, $value:ident) => ($value.0);
}

macro_rules! form_fields_to_enum {
    ($struct:ident, $enum:ident,
        $($struct_field:ident: $val_type:tt => $enum_field:ident,)+) => {{

        let mut valid = Vec::new();

        $(if let Some(ref value) = $struct.$struct_field {
            valid.push($enum::$enum_field(gen_val_type!($val_type, value)));
        })+

        valid
    }};
}

#[derive(FromForm, Debug)]
pub struct DataForm {
    pub speed:          route::Speed,
    pub dep_icao:       Option<ICAO>,
    pub dep_type:       Option<airport::Type>,
    pub dep_runway_len: Option<RunwayLength>,
    pub dep_countries:  Option<Countries>,
    pub arr_icao:       Option<ICAO>,
    pub arr_type:       Option<airport::Type>,
    pub arr_runway_len: Option<RunwayLength>,
    pub arr_countries:  Option<Countries>,
    pub min_time:       Option<Time>,
    pub max_time:       Option<Time>,
    pub sorter:         route::SortBy,
}

impl ToEnum<AirportFilter> for DataForm {
    fn to_enum(&self) -> Vec<AirportFilter> {
        form_fields_to_enum!(self, AirportFilter,
            dep_type:       self => Type,
            dep_runway_len: self => RunwayLength,
            dep_countries:  self => Countries,
        )
    }
}

impl ToEnum<RouteFilter> for DataForm {
    fn to_enum(&self) -> Vec<RouteFilter> {
        form_fields_to_enum!(self, RouteFilter,
            arr_type:       self  => ArrType,
            arr_runway_len: self  => ArrRunwayLength,
            arr_countries:  self  => ArrCountries,
            min_time:       inner => MinTime,
            max_time:       inner => MaxTime,
        )
    }
}

impl<'v> FromFormValue<'v> for route::Speed {
    type Error = &'v RawStr;

    fn from_form_value(form_value: &'v RawStr) -> Result<route::Speed, &'v RawStr> {
        use self::route::Speed;

        // Knots is usually expressed as a whole number, and mach is usually expressed as a decimal.
        // Knots should also be parsed first, since integer parsing will fail if a float is passed in
        form_value
            .parse::<i32>()
            .map(|knots| Speed::Knots(knots as f32))
            .or({
                form_value
                    .parse::<f32>()
                    .map(|mach| Speed::Mach(mach))
            })
            .map_err(|_| form_value)
    }
}

impl<'v> FromFormValue<'v> for Type {
    type Error = &'v RawStr;

    fn from_form_value(form_value: &'v RawStr) -> Result<Type, &'v RawStr> {
        match form_value.as_str() {
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

impl Deref for ICAO {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug)]
pub struct Time(pub f32);

impl<'v> FromFormValue<'v> for Time {
    type Error = &'v RawStr;

    fn from_form_value(form_value: &'v RawStr) -> Result<Time, &'v RawStr> {
        let value = form_value.as_str();
        let split = value.splitn(2, "%3A") // %3A = :
                         .collect::<Vec<_>>();

        if split.len() < 2 {
            match value.parse::<f32>() {
                Ok(v)  => Ok(Time(v)),
                Err(_) => Err(form_value),
            }
        } else {
            let hour   = split[0].parse::<i32>().map_err(|_| form_value)?;
            let minute = split[1].parse::<i32>().map_err(|_| form_value)?;

            Ok(Time(hour as f32 + (minute as f32 / 60.0)))
        }
    }
}

impl<'v> FromFormValue<'v> for RunwayLength {
    type Error = &'v RawStr;

    fn from_form_value(form_value: &'v RawStr) -> Result<RunwayLength, &'v RawStr> {
        // Since slices are invovled in runway length parsing, accepting non-ascii strings
        // can cause a panic from going out of bounds with certain characters
        if form_value.is_ascii() {
            let decoded = form_value.percent_decode_lossy();
            RunwayLength::parse(&decoded).ok_or(form_value)
        } else {
            Err(form_value)
        }
    }
}

impl<'v> FromFormValue<'v> for Countries {
    type Error = &'v RawStr;

    fn from_form_value(form_value: &'v RawStr) -> Result<Countries, &'v RawStr> {
        let countries = {
            let values = form_value
                .split('+')
                .map(|s| s.into())
                .collect::<Vec<_>>();

            // We have to manually create an empty vector if an empty string is received
            if values.len() > 0 && values[0] != "" {
                values
            } else {
                Vec::new()
            }
        };

        Ok(Countries(countries))
    }
}

impl <'v> FromFormValue<'v> for route::SortBy {
    type Error = &'v RawStr;

    fn from_form_value(form_value: &'v RawStr) -> Result<route::SortBy, &'v RawStr> {
        use self::route::SortBy;
        match form_value.as_str() {
            "distance" => Ok(SortBy::Distance),
            "arr_icao" => Ok(SortBy::ArrICAO),
            _          => Err(form_value),
        }
    }
}