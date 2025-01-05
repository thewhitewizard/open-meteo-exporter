use reqwest::Error;
use serde::Deserialize;
use std::time::Duration;

const DEFAULT_FORECAST_ENDPOINT: &str = "https://api.open-meteo.com/v1/forecast";
const DEFAULT_TIMEOUT: Duration = Duration::from_millis(10000);
const DEFAULT_CONNECT_TIMEOUT: Duration = Duration::from_millis(2000);
const DEFAULT_USER_AGENT: &str = "OMClient/1.0";

#[derive(Debug, Deserialize, Clone)]
pub struct CurrentWeather { 
    pub temperature_2m: f64,
    pub relative_humidity_2m: u8,
    pub apparent_temperature: f64,
    pub precipitation: f64,
    pub precipitation_probability: f64,
    pub rain: f64,
    pub weather_code: u16,
    pub cloud_cover: u8,
    pub pressure_msl: f64,
    pub surface_pressure: f64,
    pub wind_speed_10m: f64,
    pub wind_direction_10m: f64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct WeatherResponse {
    pub current: CurrentWeather,
}

#[derive(Debug)]
pub struct OMClient {
    pub http_client: reqwest::Client,
    pub latitude: f64,
    pub longitude: f64,
    pub timezone: String,
}

impl Default for OMClient {
    fn default() -> Self {
        Self {
            http_client: reqwest::Client::builder()
                .timeout(DEFAULT_TIMEOUT)
                .connect_timeout(DEFAULT_CONNECT_TIMEOUT)
                .user_agent(DEFAULT_USER_AGENT)
                .build()
                .unwrap(),
            latitude: 45.760002, // LYON
            longitude: 4.8399997,// LYON
            timezone: "Europe/Berlin".to_string(),
        }
    }
}

impl OMClient {
    pub fn new(latitude: f64, longitude: f64, timezone: String) -> OMClient {
        Self {
            latitude,
            longitude,
            timezone,
            ..Self::default()
        }
    }

    pub async fn get_current_weather(&self) -> Result<WeatherResponse, Error> {
        let url = format!(
            "{}?latitude={}&longitude={}&current=temperature_2m,relative_humidity_2m,apparent_temperature,precipitation,precipitation_probability,rain,weather_code,cloud_cover,pressure_msl,surface_pressure,wind_speed_10m,wind_direction_10m&timezone={}",
            DEFAULT_FORECAST_ENDPOINT, self.latitude, self.longitude, self.timezone
        );

        let response = self.http_client.get(&url).send().await?;
        let weather_data = response.json::<WeatherResponse>().await?;
        Ok(weather_data)
    }
}
