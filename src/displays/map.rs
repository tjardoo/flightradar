use crossterm::{execute, terminal::Clear, terminal::ClearType};
use std::io::{stdout, Write};

use crate::{
    displays::{draw_window, Bounds, WindowDisplay},
    models::flights::Flight,
};

pub fn display_map(flights: &[Flight]) {
    let mut stdout = stdout();
    execute!(stdout, Clear(ClearType::All)).unwrap();

    let (cols, rows) = crossterm::terminal::size().unwrap_or((80, 24));
    let display = WindowDisplay::new(cols, rows, 8, 2);

    draw_window(&mut stdout, &display);

    let lat_min = flights
        .iter()
        .map(|f| f.coordinates.latitude())
        .fold(f64::INFINITY, f64::min);
    let lat_max = flights
        .iter()
        .map(|f| f.coordinates.latitude())
        .fold(f64::NEG_INFINITY, f64::max);
    let lon_min = flights
        .iter()
        .map(|f| f.coordinates.longitude())
        .fold(f64::INFINITY, f64::min);
    let lon_max = flights
        .iter()
        .map(|f| f.coordinates.longitude())
        .fold(f64::NEG_INFINITY, f64::max);

    let bounds = Bounds {
        lat_min,
        lat_max,
        lon_min,
        lon_max,
    };

    for flight in flights {
        if let Some((x, y)) = display.latlon_to_screen(flight.coordinates.clone(), &bounds) {
            let arrow = match flight.bearing {
                h if (45.0..135.0).contains(&h) => '→',
                h if (135.0..225.0).contains(&h) => '↓',
                h if (225.0..315.0).contains(&h) => '←',
                _ => '↑',
            };

            let label = format!("{} {}", arrow, flight.id);

            execute!(
                stdout,
                crossterm::cursor::MoveTo(x, y),
                crossterm::style::Print(label)
            )
            .unwrap();
        }
    }

    stdout.flush().unwrap();
}
