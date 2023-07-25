use clap::Parser;
use serde::Deserialize;

const LAT: f32 = 37.9;
const LON: f32 = 23.7;

#[derive(Parser)]
#[command(name = "forecast")]
#[command(about = "Weather in your terminal", long_about = None)]
struct Args {
    /// Number of days for the forecase
    #[arg(short, default_value_t = 0)]
    days: u8,
}

#[derive(Debug, Deserialize)]
struct CurrentWeather {
    location: Location,
    current: Current,
}

#[derive(Debug, Deserialize)]
struct Location {
    name: String,
    region: String,
    country: String,
    lat: f32,
    lon: f32,
    tz_id: String,
    localtime_epoch: u64,
    localtime: String,
}

#[derive(Debug, Deserialize)]
struct Current {
    last_updated_epoch: u64,
    last_updated: String,
    temp_c: f32,
    temp_f: f32,
    is_day: u8,
    condition: Condition,
    wind_mph: f32,
    wind_kph: f32,
    wind_degree: u16,
    wind_dir: String,
    pressure_mb: f64,
    pressure_in: f32,
    precip_mm: f32,
    precip_in: f32,
    humidity: u8,
    cloud: u8,
    feelslike_c: f32,
    feelslike_f: f32,
    vis_km: f32,
    vis_miles: f32,
    uv: f32,
    gust_mph: f32,
    gust_kph: f32,
}

#[derive(Debug, Deserialize)]
struct Condition {
    text: String,
    icon: String,
    code: u16,
}

//cargo -q run -- --help
fn main() -> Result<(), reqwest::Error> {
    dotenv::dotenv().unwrap();

    let mut api_key = None;
    for (key, value) in std::env::vars() {
        if key != "APIKEY" {
            continue;
        }
        api_key = Some(value)
    }
    if api_key.is_none() {
        panic!("need API key")
    }
    let api_key = api_key.unwrap();

    let args = Args::parse();

    let method = match args.days {
        0 => "current",
        _ => "forecast",
    };

    let days = args.days;

    let url = format!(
        "http://api.weatherapi.com/v1/{method}.json?key={api_key}&q={LAT},{LON}&aqi=no&days={days}"
    );

    let body: CurrentWeather = reqwest::blocking::get(url)?.json()?;

    println!("{:?}", body);
    Ok(())
}
