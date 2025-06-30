use crate::{
    models::location::Coordinates,
    resources::flightaware::{Altitude, FlightawareFlight},
};

#[allow(unused)]
#[derive(Debug)]
pub struct Flight {
    pub hex: String,
    pub id: String,
    pub distance_in_km: f64,
    pub coordinates: Coordinates,
    pub altitude_in_km: u32,
    pub heading: f64,
    pub bearing: f64,
}

impl Flight {
    pub fn from_flightaware(flight: FlightawareFlight, origin: &Coordinates) -> Option<Self> {
        let (lat, lon) = match (flight.lat, flight.lon) {
            (Some(lat), Some(lon)) => (lat, lon),
            _ => return None,
        };

        if flight.alt_geom.is_none() || flight.mag_heading.is_none() {
            return None;
        }

        if let Some(Altitude::Text(_)) = flight.alt_baro {
            return None;
        }

        let id = flight
            .flight
            .as_ref()
            .map(|s| s.trim().to_string())
            .unwrap_or_else(|| flight.hex.clone());

        let coordinates = Coordinates::new(lat, lon);
        let distance_in_km = origin.distance_in_km(&coordinates);
        let altitude_in_km = (flight.alt_geom.unwrap() as f64 / 1000.0) as u32;
        let bearing = origin.bearing_to(&coordinates).round();

        Some(Flight {
            hex: flight.hex,
            id,
            coordinates,
            distance_in_km,
            altitude_in_km,
            heading: flight.mag_heading.unwrap(),
            bearing,
        })
    }
}
