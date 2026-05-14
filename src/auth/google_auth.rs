use std::env;
use clap::builder::Str;
use reqwest::blocking::{Client, get};
use serde::Serialize;

fn get_env(key: &str) -> Result<String, String> {
    let val = env::var(key)
        .map_err(|_|format!("env with key: {:?} cannot be fetched", key))?;
    Ok(val)
}
#[derive(Serialize)]
struct AuthQuery{
    client_id: String,
    client_secret: String,
    redirect_uri: String,
    response_type: String,
    scope: String,
    access_type: String
}

pub fn get_refresh_token() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    
    let auth_query = AuthQuery {
        client_id: get_env("CLIENT_ID")?,
        client_secret: get_env("CLIENT_SECRET")?,
        redirect_uri: get_env("REDIRECT_URI")?,
        response_type: String::from("code"),
        scope: get_env("SCOPE")?,
        access_type: String::from("offline")
    };
    
    let resp = client.get("https://accounts.google.com/o/oauth2/v2/auth")
        .query(&auth_query)
        .send()?;
    Ok(())
} 

pub fn open_browser_auth() -> Result<(), Box<dyn std::error::Error>>{
    
    
    Ok(())
}