use std::error::Error;
use teloxide::prelude::*;
use teloxide::utils::command::BotCommand;

#[derive(BotCommand)]
#[command(rename = "lowercase", description = "These commands are supported:")]
pub enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "handle a username.")]
    Username(String),
    #[command(description = "handle a username and an age.", parse_with = "split")]
    UsernameAndAge { username: String, age: u8 },
}

pub async fn answer(
    cx: UpdateWithCx<AutoSend<Bot>, Message>,
    command: Command,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    match command {
        Command::Help => cx.answer(Command::descriptions()).await?,
        Command::Username(username) => {
            cx.answer(format!("Your username is @{}.", username))
                .await?
        }
        Command::UsernameAndAge { username, age } => {
            cx.answer(format!(
                "Your username is @{} and age is {}.",
                username, age
            ))
            .await?
        }
    };

    Ok(())
}
