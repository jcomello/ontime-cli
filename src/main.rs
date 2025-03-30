mod cli;
mod compare_args;
mod list_args;
mod tests;
use crate::cli::{Cli, Commands};
use clap::Parser;
use std::io::{self, IsTerminal};

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
        }
        Commands::Compare(args) => {
            let compare_timezones = if io::stdin().is_terminal() {
                Cli::compare_timezones(args)
            } else {
                let lines: Vec<String> = io::stdin().lines().collect::<Result<_, _>>().unwrap();
                Cli::compare_timezones(&args.stdin_or_args(lines))
            };

            println!("{0: <25} | {1: <20}", "Timezone", "Time");
            println!(
                "{0: <25} | {1: <20}",
                "_________________________", "___________________________"
            );

            for zone in compare_timezones {
                println!(
                    "{0: <25} | {1: <20}",
                    zone.0,
                    zone.1.format("%Y-%m-%d %H:%M %Z %:z")
                );
            }
        }
    }
}
