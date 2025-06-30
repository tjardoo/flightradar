use crate::models::{flights::Flight, location::Coordinates};

pub fn filter_by_window(
    flights: Vec<Flight>,
    origin: Coordinates,
    window_direction: f64,
    field_of_view: f64,
) -> Vec<Flight> {
    flights
        .into_iter()
        .filter(|flight| {
            let bearing = origin.bearing_to(&flight.coordinates);

            let mut angle_diff = (bearing - window_direction).abs();

            if angle_diff > 180.0 {
                angle_diff = 360.0 - angle_diff;
            }

            angle_diff <= (field_of_view / 2.0)
        })
        .collect()
}
