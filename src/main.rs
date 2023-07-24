use clap::Parser;

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
fn main() {
    let args = Args::parse();
    println!("{}", args.days);
}
