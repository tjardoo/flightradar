use reqwest::Client;

use crate::{
    error::AppError,
    models::{flights::Flight, location::Coordinates},
    resources::flightaware::FlightawareResponse,
};

pub struct FlightAware;

impl FlightAware {
    pub async fn raw(client: &Client) -> Result<FlightawareResponse, AppError> {
        let url = std::env::var("API_URL")?;

        let response = client
            .get(url)
            .send()
            .await?
            .json::<FlightawareResponse>()
            .await?;

        Ok(response)
    }

    pub async fn get(client: &Client) -> Result<Vec<Flight>, AppError> {
        let response = Self::raw(client).await?;

        let origin_latitude: f64 = std::env::var("LATITUDE")?.parse().unwrap();
        let origin_longitude: f64 = std::env::var("LONGITUDE")?.parse().unwrap();

        let origin = Coordinates::new(origin_latitude, origin_longitude);

        let flights: Vec<Flight> = response
            .aircraft
            .into_iter()
            .filter_map(|raw| Flight::from_flightaware(raw, &origin))
            .collect();

        Ok(flights)
    }
}
