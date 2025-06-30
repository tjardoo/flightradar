use crate::error::AppError;

pub mod list_all;
pub mod list_window;
pub mod map;
pub mod window;

struct CliConfig {
    max_distance_in_km: Option<f64>,
    window_direction: Option<f64>,
    field_of_view: Option<f64>,
}

impl CliConfig {
    fn from_args(args: &[String]) -> Result<Self, AppError> {
        let mut config = CliConfig {
            max_distance_in_km: None,
            window_direction: None,
            field_of_view: None,
        };

        for arg in args {
            if let Some(value) = arg.strip_prefix("--max-distance=") {
                config.max_distance_in_km = Some(value.parse().unwrap());
            }
            if let Some(value) = arg.strip_prefix("--window-direction=") {
                config.window_direction = Some(value.parse().unwrap());
            }
            if let Some(value) = arg.strip_prefix("--field-of-view=") {
                config.field_of_view = Some(value.parse().unwrap());
            }
        }

        Ok(config)
    }

    fn max_distance(&self) -> Result<f64, AppError> {
        if let Some(distance) = self.max_distance_in_km {
            Ok(distance)
        } else {
            let val = std::env::var("MAX_DISTANCE_IN_KM")?;
            Ok(val.parse::<f64>().unwrap())
        }
    }

    fn window_direction(&self) -> Result<f64, AppError> {
        if let Some(direction) = self.window_direction {
            Ok(direction)
        } else {
            let val = std::env::var("WINDOW_DIRECTION")?;
            Ok(val.parse::<f64>().unwrap())
        }
    }

    fn field_of_view(&self) -> Result<f64, AppError> {
        if let Some(fov) = self.field_of_view {
            Ok(fov)
        } else {
            let val = std::env::var("FIELD_OF_VIEW")?;
            Ok(val.parse::<f64>().unwrap())
        }
    }
}
