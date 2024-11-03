use clap::{Parser, Subcommand};
use chrono::DateTime;
use chrono_tz::{TZ_VARIANTS, Tz};
use crate::compare_args::CompareArgs;

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
    Compare(CompareArgs)
}

impl Cli {
    pub fn available_timezones() -> [Tz; 596] {
        TZ_VARIANTS
    }

    pub fn compare_timezones(args: &CompareArgs) -> Vec<(String, DateTime<Tz>)> {
        args.timezones_list()
    }
}
