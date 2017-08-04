use self::airport::Runway;
use ::rocket::http::RawStr;
use ::rocket::request::FromFormValue;
use ::std::ops::Deref;
use util::StringUtil;

pub mod airport;
pub mod data;
pub mod route;

macro_rules! gen_filter_type {
    ($name:ident, $form_ty:ty,
        $($struct_variant:ident => $enum_variant:ident($type:ty),)*) => {

        #[derive(Debug)]
        pub enum $name {
            $($enum_variant($type),)*
        }

        impl $name {
            fn from_form(form: &$form_ty) -> Vec<$name> {
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
pub struct DataForm {
    pub mach:           f32,
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

gen_filter_type!(AirportFilter, DataForm,
    dep_type       => Type(airport::Type),
    dep_runway_len => RunwayLength(RunwayLength),
    dep_country    => Country(String),
);

gen_filter_type!(RouteFilter, DataForm,
    arr_type       => ArrType(airport::Type),
    arr_runway_len => ArrRunwayLength(RunwayLength),
    arr_country    => ArrCountry(String),
    min_time       => MinTime(Time),
    max_time       => MaxTime(Time),
);

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

impl Deref for ICAO {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

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
    fn parse_single(input_str: &RawStr) -> Result<RunwayLength, &RawStr> {
        if input_str.len() > 3 {
            use self::RunwayLength::*;
            let value = input_str[3..].parse().map_err(|_| input_str)?;

            match &input_str[..3] {
                "%3C" => Ok(LessThan(value)),    // <
                "%3E" => Ok(GreaterThan(value)), // >
                "%3D" => Ok(EqualTo(value)),     // =
                _     => Err(input_str),
            }
        } else {
            Err(input_str)
        }
    }

    fn parse_between(input_str: &RawStr) -> Result<RunwayLength, &RawStr> {
        let values = input_str.splitn(2, '+')
                              .collect::<Vec<_>>();

        if values.len() == 2 {
            let min = values[0].parse().map_err(|_| input_str)?;
            let max = values[1].parse().map_err(|_| input_str)?;

            Ok(RunwayLength::Between(min, max))
        } else {
            Err(input_str)
        }
    }

    pub fn parse(input_str: &RawStr) -> Result<RunwayLength, &RawStr> {
        // Since slices are invovled in parsing, accepting non-ascii strings
        // can cause a panic from going out of bounds
        if input_str.is_ascii() {
            RunwayLength::parse_between(input_str)
                .or(RunwayLength::parse_single(input_str))
        } else {
            Err(input_str)
        }
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

impl<'v> FromFormValue<'v> for RunwayLength {
    type Error = &'v RawStr;

    fn from_form_value(form_value: &'v RawStr) -> Result<RunwayLength, &'v RawStr> {
        RunwayLength::parse(form_value)
    }
}