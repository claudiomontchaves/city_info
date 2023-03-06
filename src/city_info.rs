use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Location {
    pub name: String,
    pub region: String,
    pub country: String,
    pub lat: f32,
    pub lon: f32,
    pub tz_id: String,
    pub localtime_epoch: u32,
    pub localtime: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Condition {
    pub text: String,
    pub icon: String,
    pub code: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Current {
    pub last_updated_epoch: u32,
    pub last_updated: String,
    pub temp_c: f32,
    pub temp_f: f32,
    pub is_day: u32,
    pub condition: Condition,
    pub wind_mph: f32,
    pub wind_kph: f32,
    pub wind_degree: u32,
    pub wind_dir: String,
    pub pressure_mb: f32,
    pub pressure_in: f32,
    pub precip_mm: f32,
    pub precip_in: f32,
    pub humidity: u32,
    pub cloud: u32,
    pub feelslike_c: f32,
    pub feelslike_f: f32,
    pub vis_km: f32,
    pub vis_miles: f32,
    pub uv: f32,
    pub gust_mph: f32,
    pub gust_kph: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CityInfo {
    pub location: Location,
    pub current: Current,
}
