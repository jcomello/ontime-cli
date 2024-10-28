mod cli;
use clap::Parser;
use crate::cli::{Cli, Commands};

fn main() {
    let cli = Cli::parse();
    let available_timezones = cli.available_timezones();

    println!("\n");

    match &cli.command {
        Some(Commands::List) => {
            println!("AVAILABLE TIMEZONES");
            println!("-------------------");
            for timezone in available_timezones {
                println!("{}", timezone);
            }
        },
        Some(Commands::Compare { timezones, local }) => {
            println!("{0: <25} | {1: <20}", "Timezone", "Time");
            println!("{0: <25} | {1: <20}", "_________________________", "_________________________________");

            for zone in cli.timezones(&timezones, *local) {
                println!("{0: <25} | {1: <20}", zone.0, zone.1);
            }
        },
        None => {},
    }
}
