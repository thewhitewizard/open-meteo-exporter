mod metrics; 
mod state;
mod config;
mod openmeteo;
use std::{sync::Arc, time::Duration};
use crate::openmeteo::client::OMClient;
use metrics::start_http_server; 
use state::{start_refresh_loop, OMState};
use config::Config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let config = Config::load_from_file("config.toml").expect("Failed to load configuration");
    let weather_condition_config = Config::load_weather_config("weather_condition_config.json");    
    println!("{:?}", config);

    let state = Arc::new(OMState::new());
    let client = Arc::new(OMClient::new(config.openmeteo.latitude, config.openmeteo.longitude, config.openmeteo.timezone));

    let state_clone = state.clone();
    tokio::spawn(async move {
        start_refresh_loop(
            state_clone,
            client,
            Duration::from_secs(config.server.ttl),
        )
        .await;
    });

    start_http_server(state,config.server.port,weather_condition_config).await;

    Ok(())
}
