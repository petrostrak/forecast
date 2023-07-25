use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CurrentWeather {
    // location: Location,
    pub current: Current,
}

#[derive(Debug, Deserialize)]
struct Location {
    // name: String,
    // region: String,
    // country: String,
    // lat: f32,
    // lon: f32,
    // tz_id: String,
    // localtime_epoch: u64,
    // localtime: String,
}

#[derive(Debug, Deserialize)]
pub struct Current {
    // last_updated_epoch: u64,
    // last_updated: String,
    // temp_c: f32,
    // temp_f: f32,
    // is_day: u8,
    pub condition: Condition,
    // wind_mph: f32,
    // wind_kph: f32,
    // wind_degree: u16,
    // wind_dir: String,
    // pressure_mb: f64,
    // pressure_in: f32,
    // precip_mm: f32,
    // precip_in: f32,
    // humidity: u8,
    // cloud: u8,
    pub feelslike_c: f32,
    // feelslike_f: f32,
    // vis_km: f32,
    // vis_miles: f32,
    // uv: f32,
    // gust_mph: f32,
    // gust_kph: f32,
}

#[derive(Debug, Deserialize)]
pub struct Condition {
    pub text: String,
    // icon: String,
    // code: u16,
}
