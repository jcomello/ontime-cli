use clap::{Parser, Subcommand};
use chrono::Local;
use std::str::FromStr;
use chrono_tz::{TZ_VARIANTS, Tz};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Timezone to be compared
    #[clap(name="timezone")]
    #[arg(short, long)]
    pub timezone: Vec<String>,

    /// Compare timezones with local time
    #[clap(name="local")]
    #[arg(short, long, default_value_t = false)]
    pub local: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// List available timezones
    List
}

impl Cli {
    pub fn list_available_timezones(&self) {
        println!("Available Timezones");
        for timezone in TZ_VARIANTS {
            println!("{}", timezone);
        }
    }

    pub fn list_timezones(&self) {
        let now = Local::now();

        println!("{0: <20} | {1: <20}", "Timezone", "Time");
        println!("{0: <20} | {1: <20}", "____________________", "____________________________________");

        if self.local {
            println!("{0: <20} | {1: <20}", "Local", now);
        }

        for zone in &self.timezone {
            let timezone = Tz::from_str(&zone).unwrap();
            println!("{0: <20} | {1: <10}", zone, now.with_timezone(&timezone));
        }
    }
}
