use reqwest::Client;

use crate::{
    displays::table::table_display,
    error::AppError,
    filters::{
        distance::{filter_by_distance, sort_by_distance},
        view::filter_by_window,
    },
    flightaware::FlightAware,
    models::location::Coordinates,
};

struct WindowCommandConfig {
    max_distance_in_km: Option<f64>,
    window_direction: Option<f64>,
    field_of_view: Option<f64>,
}

pub async fn handle(args: &[String]) -> Result<(), AppError> {
    let config = WindowCommandConfig::from_args(args)?;
    let max_distance_in_km = config.max_distance()?;
    let window_direction = config.window_direction()?;
    let field_of_view = config.field_of_view()?;

    let client = Client::new();

    let mut flights = FlightAware::get(&client).await?;

    let latitude = std::env::var("LATITUDE")?.parse::<f64>().unwrap();
    let longitude = std::env::var("LONGITUDE")?.parse::<f64>().unwrap();
    let origin = Coordinates::new(latitude, longitude);

    flights = filter_by_distance(flights, max_distance_in_km);
    flights = filter_by_window(flights, origin, window_direction, field_of_view);
    flights = sort_by_distance(flights);

    table_display(flights);

    Ok(())
}

impl WindowCommandConfig {
    fn from_args(args: &[String]) -> Result<Self, AppError> {
        let mut config = WindowCommandConfig {
            max_distance_in_km: None,
            window_direction: None,
            field_of_view: None,
        };

        for arg in args {
            if let Some(value) = arg.strip_prefix("--max-distance=") {
                config.max_distance_in_km = Some(value.parse().unwrap());
            }
            if let Some(value) = arg.strip_prefix("--window-direction=") {
                config.window_direction = Some(value.parse().unwrap());
            }
            if let Some(value) = arg.strip_prefix("--field-of-view=") {
                config.field_of_view = Some(value.parse().unwrap());
            }
        }

        Ok(config)
    }

    fn max_distance(&self) -> Result<f64, AppError> {
        if let Some(distance) = self.max_distance_in_km {
            Ok(distance)
        } else {
            let val = std::env::var("MAX_DISTANCE_IN_KM")?;
            Ok(val.parse::<f64>().unwrap())
        }
    }

    fn window_direction(&self) -> Result<f64, AppError> {
        if let Some(direction) = self.window_direction {
            Ok(direction)
        } else {
            let val = std::env::var("WINDOW_DIRECTION")?;
            Ok(val.parse::<f64>().unwrap())
        }
    }

    fn field_of_view(&self) -> Result<f64, AppError> {
        if let Some(fov) = self.field_of_view {
            Ok(fov)
        } else {
            let val = std::env::var("FIELD_OF_VIEW")?;
            Ok(val.parse::<f64>().unwrap())
        }
    }
}
