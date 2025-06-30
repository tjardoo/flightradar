use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum AppError {
    MissingEnvFile,
    Dotenv(dotenv::Error),
    MissingEnvironmentVariable(std::env::VarError),
    Ftail(ftail::error::FtailError),
    Reqwest(reqwest::Error),
    NoCommandProvided,
    UnknownCommand(String),
    UnknownSubcommand(String, String),
}

impl Error for AppError {}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::MissingEnvFile => write!(f, "Missing .env file"),
            AppError::Dotenv(error) => write!(f, "Dotenv error: {}", error),
            AppError::MissingEnvironmentVariable(error) => {
                write!(f, "Missing environment variable: {}", error)
            }
            AppError::Ftail(error) => write!(f, "Ftail error: {}", error),
            AppError::Reqwest(error) => write!(f, "Reqwest error: {}", error),
            AppError::NoCommandProvided => write!(f, "No command provided"),
            AppError::UnknownCommand(command) => write!(f, "Unknown command: {}", command),
            AppError::UnknownSubcommand(command, subcommand) => {
                write!(
                    f,
                    "Unknown subcommand '{}' for command '{}'",
                    subcommand, command
                )
            }
        }
    }
}

impl From<dotenv::Error> for AppError {
    fn from(error: dotenv::Error) -> AppError {
        AppError::Dotenv(error)
    }
}

impl From<std::env::VarError> for AppError {
    fn from(error: std::env::VarError) -> AppError {
        AppError::MissingEnvironmentVariable(error)
    }
}

impl From<ftail::error::FtailError> for AppError {
    fn from(error: ftail::error::FtailError) -> AppError {
        AppError::Ftail(error)
    }
}

impl From<reqwest::Error> for AppError {
    fn from(error: reqwest::Error) -> AppError {
        AppError::Reqwest(error)
    }
}
