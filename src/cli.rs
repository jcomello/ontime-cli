use clap::{Parser, Subcommand};
use iana_time_zone;
use chrono::{Utc, DateTime, Duration};
use std::str::FromStr;
use chrono_tz::{TZ_VARIANTS, Tz};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
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

        /// Compare timezones in x hours in the future
        #[clap(name="from-now")]
        #[arg(short, long, default_value_t = 0)]
        from_now: i64,
    }
}

impl Cli {
    pub fn available_timezones() -> [Tz; 596] {
        TZ_VARIANTS
    }

    pub fn compare_timezones(
        timezones: &Vec<String>,
        local: bool,
        from_now: i64) -> Vec<(String, DateTime<Tz>)> {

        let current_zone = iana_time_zone::get_timezone().unwrap();
        let current_tz = Cli::get_tz(&current_zone);
        let time = Utc::now() + Duration::hours(from_now.into());
        let mut zones: Vec<(String, DateTime<Tz>)> =
            timezones
            .clone()
            .into_iter()
            .map(|zone| {
                let timezone = Cli::get_tz(&zone);

                (zone, time.with_timezone(&timezone))
            })
            .collect();

        if local {
            let local_string = format!("{} (Local)", current_zone);
            let element = (local_string, time.with_timezone(&current_tz));
            let mut new_zones: Vec<(String, DateTime<Tz>)> = vec![element];

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
