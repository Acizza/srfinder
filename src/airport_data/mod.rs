mod data;

use self::data::DataType;
use crate::path::FilePath;
use anyhow::{anyhow, Result};
use chrono::{Duration, NaiveDate, Utc};
use serde_derive::Serialize;
use std::borrow::Cow;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, BufWriter};
use std::path::{Path, PathBuf};

const BASE_URL: &str = "https://ourairports.com/data";

fn validated_dir() -> Result<PathBuf> {
    FilePath::LocalData.validated_subdir("airport_data/")
}

pub fn ensure_updated() -> Result<()> {
    let base_dir = validated_dir()?;
    let mut last_update = LastUpdate::load(&base_dir);

    if !last_update.needs_update() {
        return Ok(());
    }

    download_latest(&base_dir)?;
    last_update.set_to_today()?;

    Ok(())
}

fn download_latest<P>(dir: P) -> Result<()>
where
    P: AsRef<Path>,
{
    println!("updating airport data");

    for dtype in &DataType::ALL {
        download_file(dtype.filename(), &dir)?;
    }

    println!("finished updating airport data");
    Ok(())
}

fn download_file<S, P>(name: S, dir: P) -> Result<()>
where
    S: AsRef<str>,
    P: AsRef<Path>,
{
    let name = name.as_ref();
    let dir = dir.as_ref();

    println!(".. downloading {}", name);

    let path = dir.join(name);
    let backup = Backup::create(&path)?;

    let resp = ureq::get(&format!("{}/{}", BASE_URL, name))
        .timeout_connect(15_000)
        .timeout_read(15_000)
        .call();

    if let Some(err) = resp.synthetic_error() {
        backup.restore()?;
        return Err(anyhow!("{}", err));
    }

    let file = match File::create(path) {
        Ok(file) => file,
        Err(err) => {
            backup.restore()?;
            return Err(err.into());
        }
    };

    let mut reader = resp.into_reader();
    let mut writer = BufWriter::new(file);

    match io::copy(&mut reader, &mut writer) {
        Ok(_) => Ok(()),
        Err(err) => {
            backup.restore()?;
            Err(err.into())
        }
    }
}

struct LastUpdate(PathBuf);

impl LastUpdate {
    const DATE_FORMAT: &'static str = "%Y-%m-%d";

    fn load<P>(base_dir: P) -> Self
    where
        P: Into<PathBuf>,
    {
        let mut path = base_dir.into();
        path.push(".last_updated");

        Self(path)
    }

    fn needs_update(&self) -> bool {
        if !self.path().exists() {
            return true;
        }

        let updated_at = fs::read_to_string(self.path())
            .ok()
            .and_then(|updated_at| NaiveDate::parse_from_str(&updated_at, Self::DATE_FORMAT).ok());

        let updated_at = match updated_at {
            Some(updated_at) => updated_at,
            None => return true,
        };

        let today = Utc::now().naive_utc().date();

        if today - updated_at >= Duration::weeks(2) {
            return true;
        }

        false
    }

    fn set_to_today(&mut self) -> Result<()> {
        let today = Utc::now().naive_utc().date();
        fs::write(self.path(), today.format(Self::DATE_FORMAT).to_string())?;
        Ok(())
    }

    #[inline(always)]
    fn path(&self) -> &Path {
        &self.0
    }
}

enum Backup {
    Exists(PathBuf),
    DoesntExist,
}

impl Backup {
    fn create<P>(file: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let file = file.as_ref();

        if !file.exists() {
            return Ok(Self::DoesntExist);
        }

        let extension = match file.extension() {
            Some(ext) => format!("{}.bak", ext.to_string_lossy()).into(),
            None => Cow::Borrowed(".bak"),
        };

        let mut backup_path = PathBuf::from(file);
        backup_path.set_extension(extension.as_ref());

        fs::rename(file, &backup_path)?;

        Ok(Self::Exists(backup_path))
    }

    fn restore(self) -> Result<()> {
        match self {
            Self::Exists(path) => {
                if !path.exists() {
                    return Ok(());
                }

                let mut original_path = path.clone();
                original_path.set_extension("");

                if original_path.exists() {
                    fs::remove_file(&original_path)?;
                }

                fs::rename(path, original_path)?;
                Ok(())
            }
            Self::DoesntExist => Ok(()),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Airport {
    pub icao: String,
    #[serde(rename = "type")]
    pub class: AirportType,
    pub position: Position,
    pub runways: Vec<Runway>,
    pub frequencies: HashMap<FrequencyType, String>,
    #[serde(rename = "countryName")]
    pub country_name: String,
}

impl Airport {
    pub fn load_all() -> Result<Vec<Self>> {
        let dir = validated_dir()?;

        let airports = data::Airport::from_dir(&dir)?;
        let mut runways = data::Runway::from_dir(&dir)?;
        let mut frequencies = data::Frequency::from_dir(&dir)?;
        let countries = data::Country::from_dir(&dir)?;

        let mut results = Vec::with_capacity(airports.len());

        for airport in airports {
            let runways = match runways.remove(&airport.id) {
                Some(runways) => runways,
                None => continue,
            };

            let frequencies = match frequencies.remove(&airport.id) {
                Some(frequencies) => frequencies,
                None => HashMap::new(),
            };

            let country = match countries.iter().find(|c| c.code == airport.country_code) {
                Some(country) => country,
                None => continue,
            };

            let result = Self {
                icao: airport.icao,
                class: airport.class.into(),
                position: Position::new(airport.lat_deg, airport.lon_deg),
                runways: runways.into_iter().filter_map(Runway::from_data).collect(),
                frequencies: frequencies
                    .into_iter()
                    .map(|(freq_type, freq)| (freq_type.into(), freq.mhz))
                    .collect(),
                country_name: country.name.clone(),
            };

            results.push(result);
        }

        results.shrink_to_fit();
        Ok(results)
    }
}

#[derive(Copy, Clone, Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum AirportType {
    Large = 0,
    Medium,
    Small,
    Closed,
    Heliport,
    #[serde(rename = "seaplaneBase")]
    SeaplaneBase,
}

impl From<data::AirportType> for AirportType {
    fn from(source: data::AirportType) -> Self {
        use data::AirportType as DAT;

        match source {
            DAT::Large => Self::Large,
            DAT::Medium => Self::Medium,
            DAT::Small => Self::Small,
            DAT::Closed => Self::Closed,
            DAT::Heliport => Self::Heliport,
            DAT::SeaplaneBase => Self::SeaplaneBase,
            DAT::Other => Self::Closed,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Position {
    #[serde(rename = "latitudeDeg")]
    latitude_deg: f32,
    #[serde(rename = "longitudeDeg")]
    longitude_deg: f32,
}

impl Position {
    #[inline(always)]
    pub fn new(latitude_deg: f32, longitude_deg: f32) -> Self {
        Self {
            latitude_deg,
            longitude_deg,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Runway {
    #[serde(rename = "lengthFT")]
    pub length_ft: Option<i32>,
    #[serde(rename = "widthFT")]
    pub width_ft: Option<i32>,
    #[serde(rename = "northMarker")]
    pub north_marker: Option<RunwayMarker>,
    #[serde(rename = "southMarker")]
    pub south_marker: Option<RunwayMarker>,
}

impl Runway {
    fn from_data(source: data::Runway) -> Option<Self> {
        let north_marker = match (source.le_ident, source.le_lat_deg, source.le_lon_deg) {
            (Some(name), Some(lat), Some(lon)) => Some(RunwayMarker::new(name, lat, lon)),
            _ => None,
        };

        let south_marker = match (source.he_ident, source.he_lat_deg, source.he_lon_deg) {
            (Some(name), Some(lat), Some(lon)) => Some(RunwayMarker::new(name, lat, lon)),
            _ => None,
        };

        Some(Self {
            length_ft: source.length_ft,
            width_ft: source.width_ft,
            north_marker,
            south_marker,
        })
    }
}

#[derive(Debug, Serialize)]
pub struct RunwayMarker {
    pub name: String,
    pub position: Position,
}

impl RunwayMarker {
    #[inline(always)]
    pub fn new(name: String, lat_deg: f32, lon_deg: f32) -> Self {
        Self {
            name,
            position: Position::new(lat_deg, lon_deg),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Frequency {
    pub mhz: String,
}

impl From<data::Frequency> for Frequency {
    fn from(source: data::Frequency) -> Self {
        Self { mhz: source.mhz }
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum FrequencyType {
    Atis,
    Arrival,
    Departure,
    #[serde(rename = "arrivalDeparture")]
    ArrivalDeparture,
    Ground,
    Tower,
    Unicom,
}

impl From<data::FrequencyType> for FrequencyType {
    fn from(source: data::FrequencyType) -> Self {
        use data::FrequencyType as DFT;

        match source {
            DFT::Atis => Self::Atis,
            DFT::Arrival => Self::Arrival,
            DFT::Departure => Self::Departure,
            DFT::ArrivalDeparture => Self::ArrivalDeparture,
            DFT::Ground => Self::Ground,
            DFT::Tower => Self::Tower,
            DFT::Unicom | DFT::Other => Self::Unicom,
        }
    }
}
