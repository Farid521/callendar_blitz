use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};
use crate::auth:: {
    refresh_token,
    env
};
pub struct Callendar {
    client_id: String,
    client_secret: String,
    refresh_token: String,

    access_token: Option<String>,
    expires_at: Option<Instant>,

    http_client: Client,
}

impl Callendar {
    pub async fn new_session() {

    }
}