use super::{AirportData, AirportType, LastUpdate, Position, RunwayMarker};
use crate::path::FilePath;
use anyhow::Result;
use serde::de::{Deserialize, Deserializer, Visitor};
use serde_derive::Deserialize;
use std::collections::HashMap;
use std::convert::TryInto;
use std::fmt;
use std::fs::File;
use std::hash::Hash;
use std::path::PathBuf;
use std::result;

pub struct OurAirports {
    data_dir: PathBuf,
    last_update: LastUpdate,
}

impl OurAirports {
    pub const URL: &'static str = "https://ourairports.com/data";

    pub fn init() -> Result<Self> {
        let data_dir = Self::data_path()?;
        let last_update = LastUpdate::load(&data_dir);

        Ok(Self {
            data_dir,
            last_update,
        })
    }

    fn data_path() -> Result<PathBuf> {
        FilePath::LocalData.validated_subdir("our_airports")
    }
}

impl AirportData for OurAirports {
    fn is_up_to_date(&self) -> bool {
        !self.last_update.needs_update()
    }

    fn update(&mut self) -> Result<()> {
        for kind in &FileType::ALL {
            super::download_file(Self::URL, kind.filename(), &self.data_dir)?;
        }

        self.last_update.set_to_today()
    }

    fn load(&self) -> Result<Vec<super::Airport>> {
        let airports = Airport::from_dir(&self.data_dir)?;
        let mut runways = Runway::from_dir(&self.data_dir)?;
        let mut frequencies = Frequency::from_dir(&self.data_dir)?;
        let countries = Country::from_dir(&self.data_dir)?;

        let mut results = Vec::with_capacity(airports.len());

        for airport in airports {
            if airport.icao.len() > Airport::MAX_ICAO_LEN {
                continue;
            }

            let runways = match runways.remove(&airport.id) {
                Some(runways) => runways,
                None => continue,
            };

            let country = match countries.iter().find(|c| c.code == airport.country_code) {
                Some(country) => country,
                None => continue,
            };

            let frequencies = frequencies
                .remove(&airport.id)
                .map(|freqs| {
                    freqs
                        .into_iter()
                        .filter_map(|(kind, freq)| {
                            kind.try_into().map(|kind| (kind, freq.mhz)).ok()
                        })
                        .collect()
                })
                .unwrap_or_else(HashMap::new);

            let result = super::Airport {
                icao: airport.icao,
                class: airport.class,
                position: Position::new(airport.lat_deg, airport.lon_deg),
                runways: runways.into_iter().map(Into::into).collect(),
                frequencies,
                country_name: country.name.clone(),
            };

            results.push(result);
        }

        results.shrink_to_fit();
        results.sort_unstable_by(|x, y| x.icao.cmp(&y.icao));

        Ok(results)
    }
}

#[derive(Copy, Clone)]
enum FileType {
    Airports,
    Runways,
    Frequencies,
    Countries,
}

impl FileType {
    const ALL: [Self; 4] = [
        Self::Airports,
        Self::Runways,
        Self::Frequencies,
        Self::Countries,
    ];

    fn filename(self) -> &'static str {
        match self {
            Self::Airports => "airports.csv",
            Self::Runways => "runways.csv",
            Self::Frequencies => "airport-frequencies.csv",
            Self::Countries => "countries.csv",
        }
    }
}

trait FileSource {
    fn file_type() -> FileType;

    fn open_reader<P>(dir: P) -> Result<csv::Reader<File>>
    where
        P: Into<PathBuf>,
    {
        let mut path = dir.into();
        path.push(Self::file_type().filename());

        let reader = csv::Reader::from_path(path)?;
        Ok(reader)
    }
}

#[derive(Debug, Deserialize)]
struct Airport {
    id: i32,
    #[serde(rename = "ident")]
    icao: String,
    #[serde(rename = "type")]
    class: AirportType,
    #[serde(rename = "latitude_deg")]
    lat_deg: f32,
    #[serde(rename = "longitude_deg")]
    lon_deg: f32,
    #[serde(rename = "iso_country")]
    country_code: String,
}

impl Airport {
    const MAX_ICAO_LEN: usize = 4;

    fn from_dir<P>(dir: P) -> Result<Vec<Self>>
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

impl FileSource for Airport {
    fn file_type() -> FileType {
        FileType::Airports
    }
}

#[derive(Debug, Deserialize)]
struct Runway {
    id: i32,
    airport_ref: i32,
    length_ft: Option<u32>,
    width_ft: Option<u32>,
    le_ident: Option<String>,
    #[serde(rename = "le_latitude_deg")]
    le_lat_deg: Option<f32>,
    #[serde(rename = "le_longitude_deg")]
    le_lon_deg: Option<f32>,
    he_ident: Option<String>,
    #[serde(rename = "he_latitude_deg")]
    he_lat_deg: Option<f32>,
    #[serde(rename = "he_longitude_deg")]
    he_lon_deg: Option<f32>,
}

impl Runway {
    fn from_dir<P>(dir: P) -> Result<HashMap<i32, Vec<Self>>>
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

impl Into<super::Runway> for Runway {
    fn into(self) -> super::Runway {
        let he_marker = match (self.le_ident, self.le_lat_deg, self.le_lon_deg) {
            (Some(name), Some(lat), Some(lon)) => Some(RunwayMarker::new(name, lat, lon)),
            _ => None,
        };

        let le_marker = match (self.he_ident, self.he_lat_deg, self.he_lon_deg) {
            (Some(name), Some(lat), Some(lon)) => Some(RunwayMarker::new(name, lat, lon)),
            _ => None,
        };

        super::Runway {
            length_ft: self.length_ft,
            width_ft: self.width_ft,
            he_marker,
            le_marker,
        }
    }
}

impl FileSource for Runway {
    fn file_type() -> FileType {
        FileType::Runways
    }
}

#[derive(Debug, Deserialize)]
struct Frequency {
    id: i32,
    airport_ref: i32,
    #[serde(rename = "type")]
    freq_type: FrequencyType,
    #[serde(rename = "frequency_mhz")]
    mhz: String,
}

impl Frequency {
    fn from_dir<P>(dir: P) -> Result<HashMap<i32, HashMap<FrequencyType, Self>>>
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

impl FileSource for Frequency {
    fn file_type() -> FileType {
        FileType::Frequencies
    }
}

// TODO: implement A/G?
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
enum FrequencyType {
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
    fn from_str(value: &str) -> Self {
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
    fn is_other(self) -> bool {
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

impl TryInto<super::FrequencyType> for FrequencyType {
    type Error = ();

    fn try_into(self) -> Result<super::FrequencyType, Self::Error> {
        use super::FrequencyType as Parent;

        match self {
            Self::Atis => Ok(Parent::Atis),
            Self::Arrival => Ok(Parent::Arrival),
            Self::Departure => Ok(Parent::Departure),
            Self::ArrivalDeparture => Ok(Parent::ArrivalDeparture),
            Self::Ground => Ok(Parent::Ground),
            Self::Tower => Ok(Parent::Tower),
            Self::Unicom => Ok(Parent::Unicom),
            Self::Other => Err(()),
        }
    }
}

#[derive(Debug, Deserialize)]
struct Country {
    id: i32,
    name: String,
    code: String,
}

impl Country {
    fn from_dir<P>(dir: P) -> Result<Vec<Self>>
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

impl FileSource for Country {
    fn file_type() -> FileType {
        FileType::Countries
    }
}
