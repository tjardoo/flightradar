use reqwest::Client;

use crate::{
    commands::CliConfig,
    displays::table::display_table,
    error::AppError,
    filters::distance::{filter_by_distance, sort_by_distance},
    flightaware::FlightAware,
};

pub async fn handle(args: &[String]) -> Result<(), AppError> {
    let config = CliConfig::from_args(args)?;
    let max_distance_in_km = config.max_distance()?;

    let client = Client::new();

    let mut flights = FlightAware::get(&client).await?;

    flights = filter_by_distance(flights, max_distance_in_km);
    flights = sort_by_distance(flights);

    display_table(flights);

    Ok(())
}
