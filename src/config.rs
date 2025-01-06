use serde::Deserialize;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Deserialize,Clone)]
pub struct WeatherConditionIcon {
    pub description: String,
    pub image: String,
}

#[derive(Debug, Deserialize,Clone)]
pub struct WeatherCondition {
    pub day: WeatherConditionIcon,
    pub night: WeatherConditionIcon,
}

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

    pub fn load_weather_config(file_path: &str) -> HashMap<String, WeatherCondition> {
        let file_content = fs::read_to_string(file_path).expect("Failed to read file");
        serde_json::from_str(&file_content).expect("Failed to parse JSON")
    }
}
