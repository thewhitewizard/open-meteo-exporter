use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct OpenMeteoConfig {
    pub latitude: f64,
    pub longitude: f64,
    pub timezone: String,
}

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub port: u16,
    pub ttl: u64,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub openmeteo: OpenMeteoConfig,
    pub server: ServerConfig,
}

impl Config {
    pub fn load_from_file(path: &str) -> Result<Self, String> {
        let content = fs::read_to_string(path).map_err(|e| format!("Failed to read config file: {}", e))?;
        toml::from_str(&content).map_err(|e| format!("Failed to parse config file: {}", e))
    }
}
