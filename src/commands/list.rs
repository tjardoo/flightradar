use reqwest::Client;

use crate::{
    displays::table::display_table,
    error::AppError,
    filters::distance::{filter_by_distance, sort_by_distance},
    flightaware::FlightAware,
};

struct ListCommandConfig {
    max_distance_in_km: Option<f64>,
}

pub async fn handle(args: &[String]) -> Result<(), AppError> {
    let config = ListCommandConfig::from_args(args)?;
    let max_distance_in_km = config.max_distance()?;

    let client = Client::new();

    let mut flights = FlightAware::get(&client).await?;

    flights = filter_by_distance(flights, max_distance_in_km);
    flights = sort_by_distance(flights);

    display_table(flights);

    Ok(())
}

impl ListCommandConfig {
    fn from_args(args: &[String]) -> Result<Self, AppError> {
        let mut config = ListCommandConfig {
            max_distance_in_km: None,
        };

        for arg in args {
            if let Some(value) = arg.strip_prefix("--max-distance=") {
                config.max_distance_in_km = Some(value.parse().unwrap());
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
}
