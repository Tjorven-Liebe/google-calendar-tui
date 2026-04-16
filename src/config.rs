use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Default)]
pub struct Config {
    pub client_id: String,
    pub client_secret: String,
    pub refresh_token: String,
}

pub fn get_config_path() -> PathBuf {
    let home = std::env::var("HOME").expect("HOME nicht gesetzt");
    PathBuf::from(home).join(".config/google_creds.json")
}

pub fn load_config_or_default() -> Config {
    let path = get_config_path();
    if let Ok(content) = fs::read_to_string(path) {
        serde_json::from_str(&content).unwrap_or_default()
    } else {
        Config::default()
    }
}

pub fn save_config(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let path = get_config_path();
    // Sicherstellen, dass ~/.config existiert
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let json = serde_json::to_string_pretty(config)?;
    fs::write(path, json)?;
    Ok(())
}