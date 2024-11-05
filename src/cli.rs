use clap::{Parser, Subcommand};
use chrono::DateTime;
use chrono_tz::Tz;
use crate::compare_args::CompareArgs;
use crate::list_args::ListArgs;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// List available timezones
    List(ListArgs),

    /// Compare timezones
    Compare(CompareArgs)
}

impl Cli {
    pub fn available_timezones(args: &ListArgs) -> Vec<Tz> {
        args.available_timezones()
    }

    pub fn compare_timezones(args: &CompareArgs) -> Vec<(String, DateTime<Tz>)> {
        args.timezones_list()
    }
}
