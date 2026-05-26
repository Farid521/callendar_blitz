use crate:: {
    auth::auth_token,
    auth::refresh_token
};
use std::fs;
use std::path::PathBuf;
use std::io;

fn get_token_file_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let home_dir = directories::BaseDirs::new()
        .ok_or("Tidak dapat menemukan home directory")?
        .home_dir()
        .to_path_buf();
    
    let token_dir = home_dir.join(".callendar_blitz");

    if !token_dir.exists() {
        fs::create_dir_all(&token_dir)?;
    }
    
    Ok(token_dir.join("refresh_token.txt"))
}

pub fn save_refresh_token(token: &str) -> Result<(), Box<dyn std::error::Error>> {
    let token_path = get_token_file_path()?;
    fs::write(&token_path, token)?;
    println!("Refresh token berhasil disimpan di: {:?}", token_path);
    Ok(())
}

pub fn load_refresh_token() -> Result<String, Box<dyn std::error::Error>> {
    let token_path = get_token_file_path()?;
    
    if !token_path.exists() {
        return Err("Refresh token tidak ditemukan. Silakan lakukan autentikasi terlebih dahulu.".into());
    }
    
    let token = fs::read_to_string(&token_path)?
        .trim()
        .to_string();
    
    if token.is_empty() {
        return Err("Refresh token kosong. Silakan lakukan autentikasi ulang.".into());
    }
    
    Ok(token)
}

pub fn delete_refresh_token() -> Result<(), Box<dyn std::error::Error>> {
    let token_path = get_token_file_path()?;
    
    if token_path.exists() {
        fs::remove_file(&token_path)?;
        println!("Refresh token berhasil dihapus");
    } else {
        println!("Tidak ada refresh token yang perlu dihapus");
    }
    
    Ok(())
}

pub fn has_refresh_token() -> bool {
    if let Ok(token_path) = get_token_file_path() {
        token_path.exists()
    } else {
        false
    }
}

pub fn authenticate() -> Result<String, Box<dyn std::error::Error>> {
    if has_refresh_token() {
        println!("\n⚠️  Refresh token sudah tersimpan!");
        println!("Apakah Anda ingin melakukan autentikasi ulang? (y/n): ");
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        
        let answer = input.trim().to_lowercase();
        if answer != "y" && answer != "yes" {
            println!("Autentikasi dibatalkan. Menggunakan refresh token yang ada.");
            return load_refresh_token();
        }
        
        println!("Melanjutkan autentikasi ulang...");
    }
    
    println!("Memulai proses autentikasi...");
    
    let auth_code = auth_token::get_auth_token()?;
    let refresh_token_value = refresh_token::get_refresh_token(auth_code)?;
    
    save_refresh_token(&refresh_token_value)?;
    
    println!("Autentikasi berhasil!");
    Ok(refresh_token_value)
}

pub fn get_or_authenticate() -> Result<String, Box<dyn std::error::Error>> {
    match load_refresh_token() {
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

