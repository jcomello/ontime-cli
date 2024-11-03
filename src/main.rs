mod cli;
mod tests;
mod compare_args;
use clap::Parser;
use crate::cli::{Cli, Commands};

fn main() {
    let cli = Cli::parse();

    println!("\n");

    match &cli.command {
        Commands::List => {
            let available_timezones = Cli::available_timezones();

            println!("AVAILABLE TIMEZONES");
            println!("-------------------");
            for timezone in available_timezones {
                println!("{}", timezone);
            }
        },
        Commands::Compare(args)  => {
            println!("{0: <25} | {1: <20}", "Timezone", "Time");
            println!("{0: <25} | {1: <20}", "_________________________", "___________________________");

            for zone in Cli::compare_timezones(args) {
                println!("{0: <25} | {1: <20}", zone.0, zone.1.format("%Y-%m-%d %H:%M %Z %:z"));
            }
        },
    }
}
