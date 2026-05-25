use std::env;

pub fn get_env(key: &str) -> Result<String, String> {
    let val = env::var(key)
        .map_err(|_|format!("env with key: {:?} cannot be fetched", key))?;
    Ok(val)
}