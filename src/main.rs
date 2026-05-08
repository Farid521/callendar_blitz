use clap::{Parser, Subcommand, command};
use std::{self, error::Error, env};
use dotenv::dotenv;

mod auth;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands
}

#[derive(Subcommand)]
enum Commands {
    Status {
        status_string: String,
    },
    Auth,
    AddEvent {
        event_name: String,
        start: u32,
        end: u32,
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // check for enviroment variable
    dotenv()?;

    let cli = Cli::parse();

    match cli.command {
        Commands::Auth => {
            println!("auth triggered");
        },
        Commands::Status {status_string} => {
            println!("status trigered, status: {}", status_string)
        },
        Commands::AddEvent { event_name, start, end } => {
            println!("event name: {}, start: {}, end: {}", event_name, start, end)
        }
    }
    Ok(())
}
