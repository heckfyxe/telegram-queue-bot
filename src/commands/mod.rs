mod keyboard;

use crate::commands::keyboard::general_keyboard;
use std::error::Error;

use strum::{Display, EnumString};
use teloxide::prelude::*;

use std::str::FromStr;

#[derive(Display, Debug, PartialEq, EnumString)]
pub enum Command {
    #[strum(to_string = "/start")]
    Start,
    #[strum(serialize = "Помощь")]
    Help,
    #[strum(serialize = "Имя")]
    Username(String),
    #[strum(serialize = "Имя и возраст")]
    UsernameAndAge {
        username: String,
        age: u8,
    },
    Unknown,
}

pub async fn answer(
    cx: UpdateWithCx<AutoSend<Bot>, Message>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let text = cx.update.text().unwrap();
    let command = Command::from_str(text).unwrap_or(Command::Unknown);
    match command {
        Command::Start => {
            cx.answer("Вставай, самурай, и занимай очередь!")
                .reply_markup(general_keyboard())
                .await?
        }
        Command::Help => cx.answer("Help message").await?,
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
        Command::Unknown => cx.answer("Не понимаю:(").await?,
    };

    Ok(())
}
