use std::sync::Arc;
use tokio::signal;
use warp::Filter;

use crate::state::OMState;

pub async fn start_http_server(state: Arc<OMState>, port: u16) {
    let metrics_route = warp::path("metrics").map(move || generate_metrics(state.clone()));

    let shutdown_signal = async {
        signal::ctrl_c()
            .await
            .expect("Failed to listen for Ctrl+C signal");
        println!("Received termination signal, shutting down gracefully.");
    };

    println!("Starting server on port {}...", port);
    let (_, server) = warp::serve(metrics_route)
        .bind_with_graceful_shutdown(([0, 0, 0, 0], port), shutdown_signal);

    server.await;
}

fn generate_metrics(state: Arc<OMState>) -> String {
    let mut metrics = String::new();
    
    let weather_data = state.get_weather();
    if let Some(weather) = weather_data {
        metrics.push_str(
            "# HELP weather_temperature_2m Current air temperature at 2 meters above ground in °C.\n",
        );
        metrics.push_str("# TYPE weather_temperature_2m gauge\n");
        metrics.push_str(&format!(
            "weather_temperature_2m {}\n",
            weather.current.temperature_2m
        ));

        metrics.push_str("# HELP weather_apparent_temperature Current apparent temperature is the perceived feels-like temperature combining wind chill factor, relative humidity and solar radiation in °C.\n");
        metrics.push_str("# TYPE weather_apparent_temperature gauge\n");
        metrics.push_str(&format!(
            "weather_apparent_temperature {}\n",
            weather.current.apparent_temperature
        ));

        metrics.push_str("# HELP weather_rain Rain from large scale weather systems of the preceding hour in millimeter.\n");
        metrics.push_str("# TYPE weather_rain gauge\n");
        metrics.push_str(&format!("weather_rain {}\n", weather.current.rain));

        metrics.push_str("# HELP weather_precipitation Total precipitation (rain, showers, snow) sum of the preceding hour in millimeter.\n");
        metrics.push_str("# TYPE weather_precipitation gauge\n");
        metrics.push_str(&format!(
            "weather_precipitation {}\n",
            weather.current.precipitation
        ));

        metrics.push_str("# HELP weather_precipitation_probability Probability of precipitation with more than 0.1 mm of the preceding hour in %.\n");
        metrics.push_str("# TYPE weather_precipitation_probability gauge\n");
        metrics.push_str(&format!(
            "weather_precipitation_probability {}\n",
            weather.current.precipitation_probability
        ));

        metrics.push_str("# HELP weather_weather_code Weather condition as a numeric code. Follow WMO weather interpretation codes.\n");
        metrics.push_str("# TYPE weather_weather_code counter\n");
        metrics.push_str(&format!("weather_weather_code {}\n", weather.current.weather_code));

        metrics.push_str("# HELP weather_relative_humidity_2m Current relative humidity at 2 meters above ground in %.\n");
        metrics.push_str("# TYPE weather_relative_humidity_2m gauge\n");
        metrics.push_str(&format!(
            "weather_relative_humidity_2m {}\n",
            weather.current.relative_humidity_2m
        ));

        metrics.push_str("# HELP weather_wind_speed_10m Wind speed at 10m in km/h.\n");
        metrics.push_str("# TYPE weather_wind_speed_10m gauge\n");
        metrics.push_str(&format!(
            "weather_wind_speed_10m {}\n",
            weather.current.wind_speed_10m
        ));

        metrics.push_str("# HELP weather_wind_direction_10m Wind direction at 10m in degrees.\n");
        metrics.push_str("# TYPE weather_wind_direction_10m gauge\n");
        metrics.push_str(&format!(
            "weather_wind_direction_10m {}\n",
            weather.current.wind_direction_10m
        ));

        metrics.push_str("# HELP weather_cloud_cover Cloud cover in %.\n");
        metrics.push_str("# TYPE weather_cloud_cover gauge\n");
        metrics.push_str(&format!("weather_cloud_cover {}\n", weather.current.cloud_cover));

        metrics.push_str("# HELP weather_surface_pressure Pressure at mean sea level in hPa.\n");
        metrics.push_str("# TYPE weather_surface_pressure gauge\n");
        metrics.push_str(&format!(
            "weather_surface_pressure {}\n",
            weather.current.surface_pressure
        ));

        metrics.push_str("# HELP weather_pressure_msl Pressure at mean sea level in hPa.\n");
        metrics.push_str("# TYPE weather_pressure_msl gauge\n");
        metrics.push_str(&format!("weather_pressure_msl {}\n", weather.current.pressure_msl));
    } else {
        metrics.push_str("# No weather data available yet.\n");
    }

    metrics
}
