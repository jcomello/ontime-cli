mod cli;
use clap::Parser;
use crate::cli::{Cli, Commands};

fn main() {
    let cli = Cli::parse();

    println!("\n");

    match &cli.command {
        Some(Commands::List) => cli.list_available_timezones(),
        None => cli.list_timezones(),
    }
}
