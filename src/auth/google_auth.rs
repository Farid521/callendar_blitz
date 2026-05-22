use std::env;
use reqwest::blocking::Client;
use serde::Serialize;
use webbrowser;
use crate::auth::listener_server::wait_for_auth_code;

fn get_env(key: &str) -> Result<String, String> {
    let val = env::var(key)
        .map_err(|_|format!("env with key: {:?} cannot be fetched", key))?;
    Ok(val)
}
#[derive(Serialize)]
struct AuthQuery{
    client_id: String,
    redirect_uri: String,
    response_type: String,
    scope: String,
    access_type: String
}

pub fn open_in_browser(url: &str) -> bool {
    if webbrowser::open(url).is_ok() {
        return true;
    }
    return false;
}

pub fn get_refresh_token() -> Result<(), Box<dyn std::error::Error>> {
    let listener_host = get_env("LISTENER_SERVER_HOST")?;
    let listener_port = get_env("LISTENER_SERVER_PORT")?;

    let client = Client::new();
    
    let auth_query = AuthQuery {
        client_id: get_env("CLIENT_ID")?,
        redirect_uri: get_env("REDIRECT_URI")?,
        response_type: String::from("code"),
        scope: get_env("SCOPES")?,
        access_type: String::from("offline")
    };
    
    let req_url = client.get("https://accounts.google.com/o/oauth2/v2/auth")
        .query(&auth_query)
        .build()?;
    let oauth_google_url = req_url.url().as_str();

    if !open_in_browser(oauth_google_url) {
        return Err("cannot launch a browser".into());
    }

    let auth_code = wait_for_auth_code(&listener_host, &listener_port)?;
    println!("Auth code diterima: {}", auth_code);

    Ok(())
} 