use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::time::interval;

use crate::openmeteo::client::WeatherResponse;
use crate::openmeteo::OMClient;

#[derive(Debug)]
pub struct OMState {
    pub weather: Mutex<Option<WeatherResponse>>,
}

impl OMState {
    pub fn new() -> Self {
        Self {
            weather: Mutex::new(None),
        }
    }

    pub fn update_weather(&self, new_weather: WeatherResponse) {
        let mut weather_lock = self.weather.lock().unwrap();
        *weather_lock = Some(new_weather);
    }

    pub fn get_weather(&self) -> Option<WeatherResponse> {
        let weather_lock = self.weather.lock().unwrap();
        weather_lock.clone()
    }
}

pub async fn start_refresh_loop(
    om_state: Arc<OMState>,
    client: Arc<OMClient>,
    refresh_interval: Duration,
) {
    let mut ticker = interval(refresh_interval);

    loop {
        ticker.tick().await;
        match client.get_current_weather().await {
            Ok(weather) => {
                om_state.update_weather(weather);
                println!("[{}] Weather updated.",chrono::offset::Utc::now().format("%Y-%m-%d %H:%M:%S"));
            }
            Err(err) => {
                eprintln!("[{}] Error refreshing weather data: {}",chrono::offset::Utc::now().format("%Y-%m-%d %H:%M:%S"), err);
            }
        }
    }
}
