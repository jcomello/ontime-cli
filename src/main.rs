use clap::Parser;
use chrono::Local;
use std::str::FromStr;
use chrono_tz::Tz;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[clap(name="city")]
    #[arg(short, long)]
    city: Vec<String>,

    #[clap(name="local")]
    #[arg(short, long, default_value_t = false)]
    local: bool,
}

fn main() {
    let args = Args::parse();
    let now = Local::now();

    println!("{0: <20} | {1: <20}", "City", "Time");
    println!("{0: <20} | {1: <20}", "____________________", "____________________________________");

    if args.local {
        println!("{0: <20} | {1: <20}", "Local", now);
    }

    for zone in args.city {
        let timezone = Tz::from_str(&zone).unwrap();
        println!("{0: <20} | {1: <10}", zone, now.with_timezone(&timezone));
    }
}
