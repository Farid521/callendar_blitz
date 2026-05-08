use std::env;
use clap::builder::Str;
use reqwest::{self, Client};
use serde::Serialize;

fn get_env(key: &str) -> Result<String, String> {
    let val = env::var(key)
        .map_err(|_|format!("env with key: {:?} cannot be fetched", key))?;
    Ok(val)
}

#[derive(Serialize)]
struct AuthParams {
    client_id: String,
    redirect_uri: String,
    response_type: String,
    scope: String,
    acces_type: String
}

pub fn get_refresh_token() -> Result<(), String> {
    let client = Client::new();

    let client_id = get_env("CLIENT_ID")?;
    
    let res = client.get("https://accounts.google.com/o/oauth2/v2/auth")
        .send();

    Ok(())
} 