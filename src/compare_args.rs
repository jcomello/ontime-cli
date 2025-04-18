use chrono::{DateTime, Duration, Utc};
use chrono_tz::Tz;
use clap::Args;
use std::str::FromStr;
pub mod errors;
use dateparser::DateTimeUtc;
use anyhow::Error;

#[derive(Debug, Args)]
pub struct CompareArgs {
    /// Timezones to be compared
    #[clap(name = "timezone")]
    #[arg(short, long)]
    pub timezones: Vec<String>,

    /// Compare timezones with local time
    #[clap(name = "local")]
    #[arg(short, long, default_value_t = false)]
    pub local: bool,

    /// Compare timezones in x hours in the future
    #[clap(name = "from-now")]
    #[arg(short, long, default_value_t = 0)]
    pub from_now: i64,

    /// Compare timezones in x hours in the past
    #[clap(name = "ago")]
    #[arg(short, long, default_value_t = 0)]
    pub ago: i64,

    /// Compare timezones at a specific datetime in commonly used formats.
    /// See all accepted formats at: https://docs.rs/dateparser/latest/dateparser/index.html#accepted-date-formats
    #[clap(name = "datetime")]
    #[arg(short, long, value_parser = parse_datetime, default_value_t = Utc::now())]
    pub datetime: DateTime<Utc>,
}

fn parse_datetime(arg: &str) -> Result<DateTime<Utc>, Error> {
    let parsed_datetime: DateTime<Utc> = match arg.parse::<DateTimeUtc>() {
        Ok(datetime) => datetime.0,
        Err(e) => return Err(e),
    };

    Ok(parsed_datetime)
}

impl CompareArgs {
    pub fn time(&self) -> DateTime<Utc> {
        self.datetime + Duration::hours(self.from_now) - Duration::hours(self.ago)
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

        let mut zones_list: Vec<(String, DateTime<Tz>)> = self
            .timezones
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

    pub fn stdin_or_args(&self, timezone_lines: Vec<String>) -> CompareArgs {
        let comparing_timezones = [timezone_lines, self.timezones.clone()].concat();

        CompareArgs {
            from_now: self.from_now,
            ago: self.ago,
            local: self.local,
            timezones: comparing_timezones,
            datetime: self.datetime,
        }
    }

    fn get_tz(timezone: &str) -> Tz {
        Tz::from_str(timezone).expect(errors::UNEXISTENT_TZ)
    }
}
