use ::std::cmp::Ordering;
use ::rocket::http::RawStr;
use ::rocket::request::FromFormValue;
use ::airport::{self, ICAO, Runway};

macro_rules! enum_with_form_parser {
    (name = $name:ident, form = $form_ty:ty,
        $($struct_variant:ident => $enum_variant:ident($type:ty),)*) => {

        #[derive(Debug)]
        pub enum $name {
            $($enum_variant($type),)*
        }

        impl $name {
            pub fn from_form(form: &$form_ty) -> Vec<$name> {
                let mut found = Vec::new();

                $(match form.$struct_variant {
                    Some(ref v) => found.push($name::$enum_variant(v.clone())),
                    None => (),
                })*

                found
            }
        }
    };
}

#[derive(FromForm, Debug)]
pub struct FilterForm {
    pub mach: f32,
    pub dep_icao:       Option<ICAO>,
    pub dep_type:       Option<airport::Type>,
    pub dep_runway_len: Option<RunwayLength>,
    pub dep_country:    Option<String>,
    pub arr_icao:       Option<ICAO>,
    pub arr_type:       Option<airport::Type>,
    pub arr_runway_len: Option<RunwayLength>,
    pub arr_country:    Option<String>,
    pub min_time:       Option<Time>,
    pub max_time:       Option<Time>,
}

enum_with_form_parser!(
    name = AirportFilter,
    form = FilterForm,
        dep_type       => Type(airport::Type),
        dep_runway_len => RunwayLength(RunwayLength),
        dep_country    => Country(String),
);

enum_with_form_parser!(
    name = RouteFilter,
    form = FilterForm,
        arr_type       => ArrType(airport::Type),
        arr_runway_len => ArrRunwayLength(RunwayLength),
        arr_country    => ArrCountry(String),
        min_time       => MinTime(Time),
        max_time       => MaxTime(Time),
);

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct RunwayLength {
    order: Ordering,
    pub value: u32,
}

impl<'v> FromFormValue<'v> for RunwayLength {
    type Error = &'v RawStr;

    // TODO: make unicode safe
    fn from_form_value(form_value: &'v RawStr) -> Result<RunwayLength, &'v RawStr> {
        if form_value.len() < 4 {
            Err(form_value)
        } else {
            let order = match &form_value[..3] {
                "%3C" => Ordering::Less,    // <
                "%3E" => Ordering::Greater, // >
                "%3D" => Ordering::Equal,   // =
                _     => Err(form_value)?,
            };

            let value = form_value[3..].parse().map_err(|_| form_value)?;

            Ok(RunwayLength {
                order,
                value
            })
        }
    }
}

impl RunwayLength {
    pub fn is_match(&self, runway: &Runway) -> bool {
        match runway.length {
            Some(len) => {
                match self.order {
                    Ordering::Less    => len <= self.value,
                    Ordering::Greater => len >= self.value,
                    Ordering::Equal   => len == self.value,
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