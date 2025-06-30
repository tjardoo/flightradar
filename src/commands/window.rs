use reqwest::Client;

use crate::{
    commands::CliConfig,
    displays::window::display_window,
    error::AppError,
    filters::{
        distance::{filter_by_distance, sort_by_distance},
        view::filter_by_window,
    },
    flightaware::FlightAware,
    models::location::Coordinates,
};

pub async fn handle(args: &[String]) -> Result<(), AppError> {
    let config = CliConfig::from_args(args)?;
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

    let flight = flights.first();

    display_window(flight);

    Ok(())
}
