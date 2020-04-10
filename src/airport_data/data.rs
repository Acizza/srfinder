use anyhow::Result;
use serde::de::{Deserialize, Deserializer, Visitor};
use serde::ser::{Serialize, Serializer};
use serde_derive::Deserialize;
use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::hash::Hash;
use std::path::PathBuf;
use std::result;

#[derive(Copy, Clone)]
pub enum DataType {
    Airports,
    Runways,
    Frequencies,
    Countries,
}

impl DataType {
    pub const ALL: [Self; 4] = [
        Self::Airports,
        Self::Runways,
        Self::Frequencies,
        Self::Countries,
    ];

    pub fn filename(self) -> &'static str {
        match self {
            Self::Airports => "airports.csv",
            Self::Runways => "runways.csv",
            Self::Frequencies => "airport-frequencies.csv",
            Self::Countries => "countries.csv",
        }
    }
}

pub trait DataSource {
    fn data_type() -> DataType;

    fn open_reader<P>(dir: P) -> Result<csv::Reader<File>>
    where
        P: Into<PathBuf>,
    {
        let mut path = dir.into();
        path.push(Self::data_type().filename());

        let reader = csv::Reader::from_path(path)?;
        Ok(reader)
    }
}

#[derive(Debug, Deserialize)]
pub struct Airport {
    pub id: i32,
    #[serde(rename = "ident")]
    pub icao: String,
    #[serde(rename = "type")]
    pub class: AirportType,
    #[serde(rename = "latitude_deg")]
    pub lat_deg: f32,
    #[serde(rename = "longitude_deg")]
    pub lon_deg: f32,
    #[serde(rename = "iso_country")]
    pub country_code: String,
}

impl Airport {
    pub fn from_dir<P>(dir: P) -> Result<Vec<Self>>
    where
        P: Into<PathBuf>,
    {
        let reader = Self::open_reader(dir)?;
        let mut results = Vec::new();

        for result in reader.into_deserialize() {
            let record: Self = result?;
            results.push(record);
        }

        Ok(results)
    }
}

impl DataSource for Airport {
    fn data_type() -> DataType {
        DataType::Airports
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum AirportType {
    Large = 0,
    Medium,
    Small,
    Closed,
    Heliport,
    SeaplaneBase,
    Unknown,
}

impl AirportType {
    fn from_str(value: &str) -> Self {
        match value {
            "large_airport" => Self::Large,
            "medium_airport" => Self::Medium,
            "small_airport" => Self::Small,
            "closed" => Self::Closed,
            "heliport" => Self::Heliport,
            "seaplane_base" => Self::SeaplaneBase,
            _ => Self::Unknown,
        }
    }
}

impl Default for AirportType {
    fn default() -> Self {
        Self::Unknown
    }
}

impl<'de> Deserialize<'de> for AirportType {
    fn deserialize<D>(deserializer: D) -> result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ArptTypeVisitor;

        impl<'de> Visitor<'de> for ArptTypeVisitor {
            type Value = AirportType;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("an airport type")
            }

            fn visit_str<E>(self, value: &str) -> result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(AirportType::from_str(value))
            }
        }

        deserializer.deserialize_str(ArptTypeVisitor)
    }
}

impl Serialize for AirportType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let value = match self {
            Self::Large => "large",
            Self::Medium => "medium",
            Self::Small => "small",
            Self::Closed => "closed",
            Self::Heliport => "heliport",
            Self::SeaplaneBase => "seaplaneBase",
            Self::Unknown => "unknown",
        };

        serializer.serialize_str(value)
    }
}

#[derive(Debug, Deserialize)]
pub struct Runway {
    pub id: i32,
    pub airport_ref: i32,
    pub length_ft: Option<i32>,
    pub width_ft: Option<i32>,
    pub le_ident: Option<String>,
    #[serde(rename = "le_latitude_deg")]
    pub le_lat_deg: Option<f32>,
    #[serde(rename = "le_longitude_deg")]
    pub le_lon_deg: Option<f32>,
    pub he_ident: Option<String>,
    #[serde(rename = "he_latitude_deg")]
    pub he_lat_deg: Option<f32>,
    #[serde(rename = "he_longitude_deg")]
    pub he_lon_deg: Option<f32>,
}

impl Runway {
    pub fn from_dir<P>(dir: P) -> Result<HashMap<i32, Vec<Self>>>
    where
        P: Into<PathBuf>,
    {
        let reader = Self::open_reader(dir)?;
        let mut results = HashMap::new();

        for result in reader.into_deserialize() {
            let record: Self = result?;

            let entry = results
                .entry(record.airport_ref)
                .or_insert_with(|| Vec::with_capacity(1));

            entry.push(record);
        }

        Ok(results)
    }
}

impl DataSource for Runway {
    fn data_type() -> DataType {
        DataType::Runways
    }
}

#[derive(Debug, Deserialize)]
pub struct Frequency {
    pub id: i32,
    pub airport_ref: i32,
    #[serde(rename = "type")]
    pub freq_type: FrequencyType,
    #[serde(rename = "frequency_mhz")]
    pub mhz: String,
}

impl Frequency {
    pub fn from_dir<P>(dir: P) -> Result<HashMap<i32, HashMap<FrequencyType, Self>>>
    where
        P: Into<PathBuf>,
    {
        let reader = Self::open_reader(dir)?;
        let mut results = HashMap::new();

        for result in reader.into_deserialize() {
            let record: Self = result?;

            if record.freq_type.is_other() {
                continue;
            }

            let entry = results
                .entry(record.airport_ref)
                .or_insert_with(|| HashMap::with_capacity(1));

            entry.insert(record.freq_type, record);
        }

        Ok(results)
    }
}

impl DataSource for Frequency {
    fn data_type() -> DataType {
        DataType::Frequencies
    }
}

// TODO: implement A/G?
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum FrequencyType {
    Atis,
    Arrival,
    Departure,
    ArrivalDeparture,
    Ground,
    Tower,
    Unicom,
    Other,
}

impl FrequencyType {
    pub fn from_str(value: &str) -> Self {
        match value {
            "ATIS" => Self::Atis,
            "APP" | "ARR" => Self::Arrival,
            "DEP" => Self::Departure,
            "A/D" => Self::ArrivalDeparture,
            "GND" | "GROUND" => Self::Ground,
            "TWR" | "TOWER" => Self::Tower,
            "UNIC" | "UNICOM" => Self::Unicom,
            _ => Self::Other,
        }
    }

    #[inline(always)]
    pub fn is_other(self) -> bool {
        self == Self::Other
    }
}

impl<'de> Deserialize<'de> for FrequencyType {
    fn deserialize<D>(deserializer: D) -> result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FreqTypeVisitor;

        impl<'de> Visitor<'de> for FreqTypeVisitor {
            type Value = FrequencyType;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a common frequency type")
            }

            fn visit_str<E>(self, value: &str) -> result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                let freq_type = FrequencyType::from_str(value);
                Ok(freq_type)
            }
        }

        deserializer.deserialize_str(FreqTypeVisitor)
    }
}

#[derive(Debug, Deserialize)]
pub struct Country {
    pub id: i32,
    pub name: String,
    pub code: String,
}

impl Country {
    pub fn from_dir<P>(dir: P) -> Result<Vec<Self>>
    where
        P: Into<PathBuf>,
    {
        let reader = Self::open_reader(dir)?;
        let mut results = Vec::new();

        for result in reader.into_deserialize() {
            let record: Self = result?;
            results.push(record);
        }

        Ok(results)
    }
}

impl DataSource for Country {
    fn data_type() -> DataType {
        DataType::Countries
    }
}
