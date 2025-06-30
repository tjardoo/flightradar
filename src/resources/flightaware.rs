use serde::Deserialize;

#[allow(unused)]
#[derive(Debug, Deserialize)]
pub struct FlightawareResponse {
    pub now: f64,
    pub messages: u32,
    pub aircraft: Vec<FlightawareFlight>,
}

#[allow(unused)]
#[derive(Debug, Deserialize)]
pub struct FlightawareFlight {
    pub hex: String,            // aircraft hex code
    pub flight: Option<String>, // flight number

    pub lat: Option<f64>, // latitude in degrees
    pub lon: Option<f64>, // longitude in degrees

    pub gs: Option<f64>,  // ground speed in knots
    pub tas: Option<f64>, // true airspeed
    pub ias: Option<f64>, // indicated airspeed
    pub mach: Option<f64>,

    pub alt_baro: Option<Altitude>,    // barometric altitude in feet
    pub alt_geom: Option<i32>,         // geometric altitude
    pub nav_altitude_mcp: Option<i32>, // MCP selected altitude

    pub track: Option<f64>,       // track angle in degrees
    pub mag_heading: Option<f64>, // magnetic heading
    pub track_rate: Option<f64>,  // rate of change of track angle
    pub roll: Option<f64>,        // roll angle

    pub seen_pos: Option<f64>, // seconds since last position update
    pub seen: Option<f64>,     // seconds since last message
    pub messages: Option<u32>, // message count

    pub rssi: Option<f64>, // signal strength
}

#[allow(unused)]
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Altitude {
    Feet(i32),
    Text(String),
}
