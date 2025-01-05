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
            "# HELP temperature_2m Current air temperature at 2 meters above ground in °C.\n",
        );
        metrics.push_str("# TYPE temperature_2m gauge\n");
        metrics.push_str(&format!(
            "temperature_2m {}\n",
            weather.current.temperature_2m
        ));

        metrics.push_str("# HELP apparent_temperature Current apparent temperature is the perceived feels-like temperature combining wind chill factor, relative humidity and solar radiation in °C.\n");
        metrics.push_str("# TYPE apparent_temperature gauge\n");
        metrics.push_str(&format!(
            "apparent_temperature {}\n",
            weather.current.apparent_temperature
        ));

        metrics.push_str("# HELP rain Rain from large scale weather systems of the preceding hour in millimeter.\n");
        metrics.push_str("# TYPE rain gauge\n");
        metrics.push_str(&format!("rain {}\n", weather.current.rain));

        metrics.push_str("# HELP precipitation Total precipitation (rain, showers, snow) sum of the preceding hour in millimeter.\n");
        metrics.push_str("# TYPE precipitation gauge\n");
        metrics.push_str(&format!(
            "precipitation {}\n",
            weather.current.precipitation
        ));

        metrics.push_str("# HELP precipitation_probability Probability of precipitation with more than 0.1 mm of the preceding hour in %.\n");
        metrics.push_str("# TYPE precipitation_probability gauge\n");
        metrics.push_str(&format!(
            "precipitation_probability {}\n",
            weather.current.precipitation_probability
        ));

        metrics.push_str("# HELP weather_code Weather condition as a numeric code. Follow WMO weather interpretation codes.\n");
        metrics.push_str("# TYPE weather_code counter\n");
        metrics.push_str(&format!("weather_code {}\n", weather.current.weather_code));

        metrics.push_str("# HELP relative_humidity_2m Current relative humidity at 2 meters above ground in %.\n");
        metrics.push_str("# TYPE relative_humidity_2m gauge\n");
        metrics.push_str(&format!(
            "relative_humidity_2m {}\n",
            weather.current.relative_humidity_2m
        ));

        metrics.push_str("# HELP wind_speed_10m Wind speed at 10m in km/h.\n");
        metrics.push_str("# TYPE wind_speed_10m gauge\n");
        metrics.push_str(&format!(
            "wind_speed_10m {}\n",
            weather.current.wind_speed_10m
        ));

        metrics.push_str("# HELP wind_direction_10m Wind direction at 10m in degrees.\n");
        metrics.push_str("# TYPE wind_direction_10m gauge\n");
        metrics.push_str(&format!(
            "wind_direction_10m {}\n",
            weather.current.wind_direction_10m
        ));

        metrics.push_str("# HELP cloud_cover Cloud cover in %.\n");
        metrics.push_str("# TYPE cloud_cover gauge\n");
        metrics.push_str(&format!("cloud_cover {}\n", weather.current.cloud_cover));

        metrics.push_str("# HELP surface_pressure Pressure at mean sea level in hPa.\n");
        metrics.push_str("# TYPE surface_pressure gauge\n");
        metrics.push_str(&format!(
            "surface_pressure {}\n",
            weather.current.surface_pressure
        ));

        metrics.push_str("# HELP pressure_msl Pressure at mean sea level in hPa.\n");
        metrics.push_str("# TYPE pressure_msl gauge\n");
        metrics.push_str(&format!("pressure_msl {}\n", weather.current.pressure_msl));
    } else {
        metrics.push_str("# No weather data available yet.\n");
    }

    metrics
}
