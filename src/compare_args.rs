use clap::Args;
use chrono::{Utc, DateTime, Duration};
use chrono_tz::Tz;
use std::str::FromStr;

#[derive(Debug, Args)]
pub struct CompareArgs {
    /// Timezones to be compared
    #[clap(name="timezone")]
    #[arg(short, long, required=true)]
    pub timezones: Vec<String>,

    /// Compare timezones with local time
    #[clap(name="local")]
    #[arg(short, long, default_value_t = false)]
    pub local: bool,

    /// Compare timezones in x hours in the future
    #[clap(name="from-now")]
    #[arg(short, long, default_value_t = 0)]
    pub from_now: i64,

    /// Compare timezones in x hours in the past
    #[clap(name="ago")]
    #[arg(short, long, default_value_t = 0)]
    pub ago: i64,
}

impl CompareArgs {
    pub fn time(&self) -> DateTime<Utc> {
        Utc::now() + Duration::hours(self.from_now) - Duration::hours(self.ago)
    }

    pub fn timezones_list(&self) -> Vec<(String, DateTime<Tz>)> {
        let time = self.time();
        let mut timezones_list: Vec<(String, DateTime<Tz>)> = vec![];

        if self.local {
            let current_zone = iana_time_zone::get_timezone().unwrap();
            let current_tz = Self::get_tz(&current_zone);
            let local_string = format!("{} (Local)", current_zone);
            let element = (local_string, time.with_timezone(&current_tz));

            timezones_list.push(element)
        }

        let mut zones_list: Vec<(String, DateTime<Tz>)> =
            self.timezones
            .clone()
            .into_iter()
            .map(|zone| {
                let timezone = Self::get_tz(&zone);

                (zone, time.with_timezone(&timezone))
            })
            .collect();

        timezones_list.append(&mut zones_list);

        timezones_list
    }

    fn get_tz(timezone: &String) -> Tz {
        Tz::from_str(&timezone).unwrap()
    }
}
