use clap::{Parser, Subcommand, command};
use std::{self, error::Error, env};
use dotenv::dotenv;

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

fn required_env(key: &str) -> Result<String, String> {
    let val = env::var(key)
        .map_err(|_| format!("err while searching for {} env variable", key))?;

    Ok(val)
}

fn main() -> Result<(), Box<dyn Error>> {
    // check for enviroment variable
    dotenv()?;
    let client_id = required_env("CLIENT_ID")?;
    let client_secret = required_env("CLIENT_SECRET")?;
    let token_uri = required_env("TOKEN_URI")?;
    let auth_uri = required_env("AUTH_URI")?;
    let redirect_uri = required_env("REDIRECT_URI")?;

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
