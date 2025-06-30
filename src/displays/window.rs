use crossterm::{execute, terminal::Clear, terminal::ClearType};
use std::io::{stdout, Write};

use crate::{
    displays::{draw_window, WindowDisplay},
    models::flights::Flight,
};

pub fn display_window(flight: Option<&Flight>) {
    let mut stdout = stdout();
    execute!(stdout, Clear(ClearType::All)).unwrap();

    let (cols, rows) = crossterm::terminal::size().unwrap_or((80, 24));
    let display = WindowDisplay::new(cols, rows, 6, 2);

    draw_window(&mut stdout, &display);

    let window_direction = std::env::var("WINDOW_DIRECTION")
        .ok()
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(0.0);

    if let Some(flight) = flight {
        let relative_heading = (flight.heading - window_direction + 360.0) % 360.0;

        if (45.0..135.0).contains(&relative_heading) || (225.0..315.0).contains(&relative_heading) {
            // draw vertical line
            for (i, row) in (display.inner_top()..=display.inner_bottom()).enumerate() {
                if i % 2 == 0 {
                    execute!(
                        stdout,
                        crossterm::cursor::MoveTo(display.mid_col(), row),
                        crossterm::style::Print("│")
                    )
                    .unwrap();
                } else {
                    execute!(
                        stdout,
                        crossterm::cursor::MoveTo(display.mid_col(), row),
                        crossterm::style::Print(" ")
                    )
                    .unwrap();
                }
            }

            let arrow = if (45.0..135.0).contains(&relative_heading) {
                '↑'
            } else {
                '↓'
            };
            let arrow_pos = if arrow == '↑' {
                display.inner_top()
            } else {
                display.inner_bottom()
            };

            execute!(
                stdout,
                crossterm::cursor::MoveTo(display.mid_col(), arrow_pos),
                crossterm::style::Print(arrow)
            )
            .unwrap();
        } else {
            // draw horizontal line
            for (i, col) in (display.inner_left()..=display.inner_right()).enumerate() {
                if i % 2 == 0 {
                    execute!(
                        stdout,
                        crossterm::cursor::MoveTo(col, display.mid_row()),
                        crossterm::style::Print("─")
                    )
                    .unwrap();
                } else {
                    execute!(
                        stdout,
                        crossterm::cursor::MoveTo(col, display.mid_row()),
                        crossterm::style::Print(" ")
                    )
                    .unwrap();
                }
            }

            let arrow = if (135.0..225.0).contains(&relative_heading) {
                '←'
            } else {
                '→'
            };
            let arrow_pos = if arrow == '←' {
                display.inner_left()
            } else {
                display.inner_right()
            };

            execute!(
                stdout,
                crossterm::cursor::MoveTo(arrow_pos, display.mid_row()),
                crossterm::style::Print(arrow)
            )
            .unwrap();
        }

        draw_flight_information(&mut stdout, &display, flight);
    }

    stdout.flush().unwrap();
}

fn draw_flight_information(stdout: &mut std::io::Stdout, display: &WindowDisplay, flight: &Flight) {
    let left = display.left();
    let right = display.right();

    let width = right - left + 1;

    let info = format!(
        "{} | Distance: {} km | Altitude: {} km | Heading: {:.0}° | Bearing: {:.0}°",
        flight.id, flight.distance_in_km, flight.altitude_in_km, flight.heading, flight.bearing,
    );

    let info_len = info.chars().count() as u16;
    let info_col = if width > info_len {
        left + (width - info_len) / 2
    } else {
        left
    };

    let info_row = display.bottom() + 2;

    execute!(
        stdout,
        crossterm::cursor::MoveTo(info_col, info_row),
        crossterm::style::Print(info)
    )
    .unwrap();
}
