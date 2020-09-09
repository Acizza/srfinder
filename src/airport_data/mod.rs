pub mod our_airports;

use anyhow::{anyhow, Result};
use chrono::{Duration, NaiveDate, Utc};
use serde::de::{Deserialize, Deserializer, Visitor};
use serde::ser::{Serialize, Serializer};
use serde_derive::Serialize;
use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt;
use std::fs::{self, File};
use std::io::{self, BufWriter};
use std::path::{Path, PathBuf};
use std::result;

pub trait AirportData {
    fn is_up_to_date(&self) -> bool;
    fn update(&mut self) -> Result<()>;

    fn load(&self) -> Result<Vec<Airport>>;
}

fn download_file<S, P>(url: &str, name: S, dir: P) -> Result<()>
where
    S: AsRef<str>,
    P: AsRef<Path>,
{
    let name = name.as_ref();
    let dir = dir.as_ref();

    println!(".. downloading {}", name);

    let path = dir.join(name);
    let backup = Backup::create(&path)?;

    let resp = attohttpc::get(format!("{}/{}", url, name))
        .timeout(Duration::seconds(15).to_std().unwrap())
        .send()?;

    if !resp.is_success() {
        backup.restore()?;

        return Err(anyhow!(
            "received code {} while downloading {}",
            resp.status(),
            name
        ));
    }

    let file = match File::create(path) {
        Ok(file) => file,
        Err(err) => {
            backup.restore()?;
            return Err(err.into());
        }
    };

    let (_, _, mut reader) = resp.split();
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
    #[serde(skip_serializing)]
    pub class: AirportType,
    pub position: Position,
    pub runways: Vec<Runway>,
    pub frequencies: HashMap<FrequencyType, String>,
    #[serde(skip_serializing)]
    pub country_name: String,
}

impl PartialEq for Airport {
    fn eq(&self, other: &Self) -> bool {
        self.icao == other.icao
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum AirportType {
    Large,
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

#[derive(Copy, Clone, Debug, Serialize)]
pub struct Position {
    #[serde(rename = "latitudeDeg")]
    pub latitude_deg: f32,
    #[serde(rename = "longitudeDeg")]
    pub longitude_deg: f32,
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
    pub length_ft: Option<u32>,
    #[serde(rename = "widthFT")]
    pub width_ft: Option<u32>,
    #[serde(rename = "heMarker")]
    pub he_marker: Option<RunwayMarker>,
    #[serde(rename = "leMarker")]
    pub le_marker: Option<RunwayMarker>,
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
