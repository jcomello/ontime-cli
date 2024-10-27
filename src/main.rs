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
        None => {
            println!("{0: <20} | {1: <20}", "Timezone", "Time");
            println!("{0: <20} | {1: <20}", "____________________", "____________________________________");

            for zone in cli.timezones() {
                println!("{0: <20} | {1: <10}", zone.0, zone.1);
            }
        },
    }
}
