use clap::{Parser, Subcommand};
use chrono::Local;
use std::str::FromStr;
use chrono_tz::Tz;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Timezone to be compared
    #[clap(name="timezone")]
    #[arg(short, long)]
    timezone: Vec<String>,

    /// Compare timezones with local time
    #[clap(name="local")]
    #[arg(short, long, default_value_t = false)]
    local: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// List available timezones
    List {
        /// list all available timezones
        #[arg(short, long)]
        all: bool,
    },
}


fn main() {
    let cli = Cli::parse();
    let now = Local::now();


    println!("{0: <20} | {1: <20}", "City", "Time");
    println!("{0: <20} | {1: <20}", "____________________", "____________________________________");

    if cli.local {
        println!("{0: <20} | {1: <20}", "Local", now);
    }

    for zone in cli.timezone {
        let timezone = Tz::from_str(&zone).unwrap();
        println!("{0: <20} | {1: <10}", zone, now.with_timezone(&timezone));
    }
}
