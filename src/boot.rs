use ftail::Ftail;
use log::LevelFilter;

use crate::error::AppError;

pub fn init() -> Result<(), AppError> {
    let path = std::path::Path::new(".env");

    if !path.exists() {
        return Err(AppError::MissingEnvFile);
    }

    dotenv::dotenv()?;

    let log_level = std::env::var("LOG_LEVEL")?;

    let log_level = match log_level.as_str() {
        "trace" => LevelFilter::Trace,
        "debug" => LevelFilter::Debug,
        "info" => LevelFilter::Info,
        "warn" => LevelFilter::Warn,
        "error" => LevelFilter::Error,
        "off" => LevelFilter::Off,
        _ => LevelFilter::Info,
    };

    Ftail::new().console(log_level).init()?;

    std::env::var("LATITUDE")?;
    std::env::var("LONGITUDE")?;
    std::env::var("WINDOW_DIRECTION")?;
    std::env::var("FIELD_OF_VIEW")?;
    std::env::var("MAX_DISTANCE_IN_KM")?;
    std::env::var("API_URL")?;

    log::debug!(target: "app", "Application initialized");

    Ok(())
}
