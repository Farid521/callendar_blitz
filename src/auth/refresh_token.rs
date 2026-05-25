use serde::{
    Serialize, Deserialize
};
use reqwest::blocking;
use crate::auth::env::get_env;

#[derive(Serialize)]
struct TokenRequest {
    code: String,
    client_id: String,
    client_secret: String,
    redirect_url: String,
    grant_type: String
}

#[derive(Deserialize, Debug)]
struct TokenResponse {
    access_token: String,
    expires_in: i32,
    refresh_token: Option<String>,
    scope: String,
    token_type: String,
}

pub fn get_refresh_token (auth_code: String) -> Result<String, Box<dyn std::error::Error>> {
    let client = blocking::Client::new();
    
    let req_body = TokenRequest {
        code: auth_code,
        client_id: get_env("CLIENT_ID")?,
        client_secret: get_env("CLIENT_SECRET")?,
        redirect_url: get_env("REDIRECT_URI")?,
        grant_type: String::from("authorization_code")
    };

    let response = client
        .post("https://oauth2.googleapis.com/token")
        .form(&req_body)
        .send()?;
    
    let token: TokenResponse = response.json()?;

    token.refresh_token
        .ok_or_else(|| "Refresh token tidak ditemukan dalam response".into())
}

