use crate::models::flights::Flight;

pub fn filter_by_distance(flights: Vec<Flight>, max_distance_in_km: f64) -> Vec<Flight> {
    flights
        .into_iter()
        .filter(|flight| flight.distance_in_km <= max_distance_in_km)
        .collect()
}

pub fn sort_by_distance(flights: Vec<Flight>) -> Vec<Flight> {
    let mut sorted_flights = flights;
    sorted_flights.sort_by(|a, b| a.distance_in_km.partial_cmp(&b.distance_in_km).unwrap());
    sorted_flights
}
