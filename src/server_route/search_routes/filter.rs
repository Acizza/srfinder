use super::Time;
use crate::airport_data::AirportType;
use serde_derive::Deserialize;
use smol_str::SmolStr;

#[derive(Debug, Deserialize)]
pub struct Filters {
    pub speed: SpeedFilter,
    pub departure: Option<AirportFilters>,
    pub arrival: Option<AirportFilters>,
    #[serde(rename = "timeRange", default)]
    pub time_range: TimeRange,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
#[serde(tag = "type", content = "value")]
pub enum SpeedFilter {
    Mach(f32),
    Knots(u32),
}

#[derive(Debug, Deserialize)]
pub struct AirportFilters {
    pub icao: Option<SmolStr>,
    #[serde(rename = "airportType", default)]
    pub airport_type: AirportType,
    #[serde(rename = "runwayLength")]
    pub runway_length: Option<RunwayLength>,
    #[serde(default)]
    pub countries: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "selector", content = "length")]
pub enum RunwayLength {
    #[serde(rename = "eq")]
    Equal(u32),
    #[serde(rename = "gt")]
    GreaterThan(u32),
    #[serde(rename = "lt")]
    LessThan(u32),
}

#[derive(Debug, Default, Deserialize)]
pub struct TimeRange {
    pub min: Option<Time>,
    pub max: Option<Time>,
}
