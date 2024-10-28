use clap::{Parser, Subcommand};
use iana_time_zone;
use chrono::{Local, DateTime};
use std::str::FromStr;
use chrono_tz::{TZ_VARIANTS, Tz};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// List available timezones
    List,

    /// Compare timezones
    Compare {
        /// Timezones to be compared
        #[clap(name="timezone")]
        #[arg(short, long, required=true)]
        timezones: Vec<String>,

        /// Compare timezones with local time
        #[clap(name="local")]
        #[arg(short, long, default_value_t = false)]
        local: bool,
    }
}

impl Cli {
    pub fn available_timezones(&self) -> [Tz; 596] {
        TZ_VARIANTS
    }

    pub fn timezones(&self, timezones: &Vec<String>, local: bool) -> Vec<(String, DateTime<Tz>)> {
        let current_zone = iana_time_zone::get_timezone().unwrap();
        let current_tz = Cli::get_tz(&current_zone);
        let now = Local::now();
        let mut zones: Vec<(String, DateTime<Tz>)> = timezones.clone().into_iter().map(|zone| {
            let timezone = Cli::get_tz(&zone);

            (zone, now.with_timezone(&timezone))
        }).collect();

        if local {
            let mut new_zones: Vec<(String, DateTime<Tz>)> = vec![("Local".to_string(), now.with_timezone(&current_tz))];

            new_zones.append(&mut zones);

            return new_zones;
        } else {
            return zones;
        }
    }

    fn get_tz(timezone: &String) -> Tz {
        Tz::from_str(&timezone).unwrap()
    }
}
