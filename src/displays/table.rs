use crate::models::flights::Flight;

pub fn display_table(flights: Vec<Flight>) {
    if flights.is_empty() {
        println!("No flights found.");
    } else {
        println!(
            "{:<10} {:>10} {:>10} {:>10} {:>10}",
            "Flight", "Distance", "Altitude", "Heading", "Bearing"
        );

        for flight in flights {
            println!(
                "{:<10} {:>10} {:>10} {:>10} {:>10}",
                format!("{}", flight.id),
                format!("↔ {}km", flight.distance_in_km),
                format!("↑ {}km", flight.altitude_in_km),
                format!("↗ {}°", flight.heading),
                format!("➤  {}°", flight.bearing)
            );
        }
    }
}
