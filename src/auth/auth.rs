use crate:: {
    auth::auth_token,
    auth::refresh_token
};

pub fn utilize_session() -> Result<(), Box<dyn std::error::Error>> {
    let auth_token = auth_token::get_auth_token()?;
    let refresh_token = refresh_token::get_refresh_token(auth_token)?;
    Ok(())
}

