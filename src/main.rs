mod boot;
mod cli;
mod commands;
mod displays;
mod error;
mod filters;
mod flightaware;
mod models;
mod resources;

#[tokio::main]
async fn main() -> Result<(), error::AppError> {
    boot::init()?;

    cli::handle().await?;

    Ok(())
}
