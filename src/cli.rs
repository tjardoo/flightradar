use crate::error::AppError;

pub async fn handle() -> Result<(), AppError> {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() < 2 {
        return Err(AppError::NoCommandProvided);
    }

    let command = &args[1];

    match command.as_str() {
        "list" => crate::commands::list::handle(&args[2..]).await?,
        "window" => crate::commands::window::handle(&args[2..]).await?,
        _ => return Err(AppError::UnknownCommand(command.to_string())),
    }

    Ok(())
}
