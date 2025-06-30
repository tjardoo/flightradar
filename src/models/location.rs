#[derive(Debug, Clone)]
pub struct Coordinates {
    latitude: f64,
    longitude: f64,
}

impl Coordinates {
    pub fn new(latitude: f64, longitude: f64) -> Self {
        Coordinates {
            latitude,
            longitude,
        }
    }

    #[allow(unused)]
    pub fn latitude(&self) -> f64 {
        self.latitude
    }

    #[allow(unused)]
    pub fn longitude(&self) -> f64 {
        self.longitude
    }

    pub fn distance_in_km(&self, other: &Coordinates) -> f64 {
        const EARTH_RADIUS_KM: f64 = 6371.0;

        let lat1 = self.latitude.to_radians();
        let lon1 = self.longitude.to_radians();
        let lat2 = other.latitude.to_radians();
        let lon2 = other.longitude.to_radians();

        let dlat = lat2 - lat1;
        let dlon = lon2 - lon1;

        let a = (dlat / 2.0).sin().powi(2) + lat1.cos() * lat2.cos() * (dlon / 2.0).sin().powi(2);

        let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

        (EARTH_RADIUS_KM * c).round()
    }

    pub fn bearing_to(&self, other: &Coordinates) -> f64 {
        let lat1 = self.latitude.to_radians();
        let lat2 = other.latitude.to_radians();
        let dlon = (other.longitude - self.longitude).to_radians();

        let y = dlon.sin() * lat2.cos();
        let x = lat1.cos() * lat2.sin() - lat1.sin() * lat2.cos() * dlon.cos();

        let bearing_rad = y.atan2(x);
        let bearing_deg = bearing_rad.to_degrees();

        (bearing_deg + 360.0) % 360.0
    }
}
