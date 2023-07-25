use clap::Parser;
use models::CurrentWeather;

mod models;

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

    let weather: CurrentWeather = reqwest::blocking::get(url)?.json()?;

    println!(
        "Its {}°C outside and it is {:?}.",
        weather.current.feelslike_c, weather.current.condition.text
    );
    Ok(())
}
