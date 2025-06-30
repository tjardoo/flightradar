use crate::{
    commands::{list_all, list_window},
    error::AppError,
};

pub async fn handle() -> Result<(), AppError> {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() < 2 {
        return Err(AppError::NoCommandProvided);
    }

    let command = &args[1];

    match command.as_str() {
        "list" => match args.get(2) {
            Some(subcommand) if subcommand == "all" => list_all::handle(&args[3..]).await?,
            Some(subcommand) if subcommand == "window" => list_window::handle(&args[3..]).await?,
            _ => {
                return Err(AppError::UnknownSubcommand(
                    command.to_string(),
                    args.get(2).cloned().unwrap_or_default(),
                ))
            }
        },
        "window" => crate::commands::window::handle(&args[2..]).await?,
        "map" => crate::commands::map::handle(&args[2..]).await?,
        _ => return Err(AppError::UnknownCommand(command.to_string())),
    }

    Ok(())
}
