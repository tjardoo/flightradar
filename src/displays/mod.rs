use crossterm::execute;

use crate::models::location::Coordinates;

pub mod map;
pub mod table;
pub mod window;

struct WindowDisplay {
    cols: u16,
    rows: u16,
    horizonal_padding: u16,
    vertical_padding: u16,
}

pub struct Bounds {
    pub lat_min: f64,
    pub lat_max: f64,
    pub lon_min: f64,
    pub lon_max: f64,
}

impl WindowDisplay {
    fn new(cols: u16, rows: u16, horizonal_padding: u16, vertical_padding: u16) -> Self {
        Self {
            cols,
            rows,
            horizonal_padding,
            vertical_padding,
        }
    }

    fn top(&self) -> u16 {
        self.vertical_padding
    }

    fn left(&self) -> u16 {
        self.horizonal_padding
    }

    fn bottom(&self) -> u16 {
        self.rows.saturating_sub(self.vertical_padding) - 3
    }

    fn right(&self) -> u16 {
        self.cols.saturating_sub(self.horizonal_padding) - 1
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
        self.top() + 1 + self.vertical_padding
    }

    fn inner_left(&self) -> u16 {
        self.left() + 1 + self.horizonal_padding
    }

    fn inner_bottom(&self) -> u16 {
        self.bottom() - 1 - self.vertical_padding
    }

    fn inner_right(&self) -> u16 {
        self.right() - 1 - self.horizonal_padding
    }

    pub fn latlon_to_screen(
        &self,
        coordinates: Coordinates,
        bounds: &Bounds,
    ) -> Option<(u16, u16)> {
        let lat = coordinates.latitude();
        let lon = coordinates.longitude();

        if lat < bounds.lat_min
            || lat > bounds.lat_max
            || lon < bounds.lon_min
            || lon > bounds.lon_max
        {
            return None;
        }

        let x_ratio = (lon - bounds.lon_min) / (bounds.lon_max - bounds.lon_min);
        let y_ratio = 1.0 - (lat - bounds.lat_min) / (bounds.lat_max - bounds.lat_min);

        let x_range = self.inner_right().saturating_sub(self.inner_left()) as f64;
        let y_range = self.inner_bottom().saturating_sub(self.inner_top()) as f64;

        let x = self.inner_left() as f64 + x_ratio * x_range;
        let y = self.inner_top() as f64 + y_ratio * y_range;

        Some((x.round() as u16, y.round() as u16))
    }
}

fn draw_window(stdout: &mut std::io::Stdout, display: &WindowDisplay) {
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
