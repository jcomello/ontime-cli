mod cli;
mod tests;
mod compare_args;
mod list_args;
use clap::Parser;
use crate::cli::{Cli, Commands};

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::List(args) => {
            let available_timezones = Cli::available_timezones(args);

            println!("AVAILABLE TIMEZONES");
            println!("-------------------");
            for timezone in available_timezones {
                println!("{}", timezone);
            }
        },
        Commands::Compare(args)  => {
            let compare_timezones = Cli::compare_timezones(args);

            println!("{0: <25} | {1: <20}", "Timezone", "Time");
            println!("{0: <25} | {1: <20}", "_________________________", "___________________________");

            for zone in compare_timezones {
                println!("{0: <25} | {1: <20}", zone.0, zone.1.format("%Y-%m-%d %H:%M %Z %:z"));
            }
        },
    }
}
