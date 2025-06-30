use crossterm::{execute, terminal::Clear, terminal::ClearType};
use std::io::{stdout, Write};

use crate::models::flights::Flight;

struct FlightDisplay {
    cols: u16,
    rows: u16,
    padding: u16,
}

impl FlightDisplay {
    fn new(cols: u16, rows: u16, padding: u16) -> Self {
        Self {
            cols,
            rows,
            padding,
        }
    }

    fn top(&self) -> u16 {
        self.padding
    }

    fn left(&self) -> u16 {
        self.padding
    }

    fn bottom(&self) -> u16 {
        self.rows.saturating_sub(self.padding) - 3
    }

    fn right(&self) -> u16 {
        self.cols.saturating_sub(self.padding) - 1
    }

    fn width(&self) -> u16 {
        self.right() - self.left() + 2
    }

    fn height(&self) -> u16 {
        self.bottom() - self.top() + 3
    }

    fn mid_row(&self) -> u16 {
        (self.top() + self.bottom()) / 2
    }

    fn mid_col(&self) -> u16 {
        (self.left() + self.right()) / 2
    }

    fn inner_top(&self) -> u16 {
        self.top() + 1 + self.padding
    }

    fn inner_left(&self) -> u16 {
        self.left() + 1 + self.padding
    }

    fn inner_bottom(&self) -> u16 {
        self.bottom() - 1 - self.padding
    }

    fn inner_right(&self) -> u16 {
        self.right() - 1 - self.padding
    }
}

pub fn display_map(flight: Option<&Flight>) {
    let mut stdout = stdout();
    execute!(stdout, Clear(ClearType::All)).unwrap();

    let (cols, rows) = crossterm::terminal::size().unwrap_or((80, 24));
    let display = FlightDisplay::new(cols, rows, 2);

    draw_window(&mut stdout, &display);

    if let Some(flight) = flight {
        let heading = flight.heading % 360.0;

        if (45.0..135.0).contains(&heading) || (225.0..315.0).contains(&heading) {
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

            let arrow = if (45.0..135.0).contains(&heading) {
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

            let arrow = if (135.0..225.0).contains(&heading) {
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

fn draw_window(stdout: &mut std::io::Stdout, display: &FlightDisplay) {
    let top = display.top();
    let left = display.left();
    let width = display.width();
    let height = display.height();

    for x in left..left + width {
        execute!(
            stdout,
            crossterm::cursor::MoveTo(x, top),
            crossterm::style::Print("─"),
            crossterm::cursor::MoveTo(x, top + height - 1),
            crossterm::style::Print("─")
        )
        .unwrap();
    }

    for y in top + 1..top + height - 1 {
        execute!(
            stdout,
            crossterm::cursor::MoveTo(left, y),
            crossterm::style::Print("│"),
            crossterm::cursor::MoveTo(left + width - 1, y),
            crossterm::style::Print("│")
        )
        .unwrap();
    }

    execute!(
        stdout,
        crossterm::cursor::MoveTo(left, top),
        crossterm::style::Print("┌"),
        crossterm::cursor::MoveTo(left + width - 1, top),
        crossterm::style::Print("┐"),
        crossterm::cursor::MoveTo(left, top + height - 1),
        crossterm::style::Print("└"),
        crossterm::cursor::MoveTo(left + width - 1, top + height - 1),
        crossterm::style::Print("┘")
    )
    .unwrap();
}

fn draw_flight_information(stdout: &mut std::io::Stdout, display: &FlightDisplay, flight: &Flight) {
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
