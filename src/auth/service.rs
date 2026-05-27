use crate:: {
    auth::auth_token,
    auth::refresh_token,
    auth::storage
};
use std::fs;
use std::path::PathBuf;
use std::io;

pub fn authenticate() -> Result<String, Box<dyn std::error::Error>> {
    if storage::has_refresh_token() {
        println!("\n⚠️  Refresh token sudah tersimpan!");
        println!("Apakah Anda ingin melakukan autentikasi ulang? (y/n): ");
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        
        let answer = input.trim().to_lowercase();
        if answer != "y" && answer != "yes" {
            println!("Autentikasi dibatalkan. Menggunakan refresh token yang ada.");
            return storage::load_refresh_token();
        }
        
        println!("Melanjutkan autentikasi ulang...");
    }
    
    println!("Memulai proses autentikasi...");
    
    let auth_code = auth_token::get_auth_token()?;
    let refresh_token_value = refresh_token::get_refresh_token(auth_code)?;
    
    storage::save_refresh_token(&refresh_token_value)?;
    
    println!("Autentikasi berhasil!");
    Ok(refresh_token_value)
}

pub fn get_or_authenticate() -> Result<String, Box<dyn std::error::Error>> {
    match storage::load_refresh_token() {
        Ok(token) => {
            println!("Menggunakan refresh token yang tersimpan");
            Ok(token)
        },
        Err(_) => {
            println!("Refresh token tidak ditemukan, memulai autentikasi baru...");
            authenticate()
        }
    }
}

