use std::error::Error;

use crate::database::DATABASE;
use teloxide::prelude::*;
use teloxide::utils::command::BotCommand;

use crate::keyboard::general_keyboard;

#[derive(BotCommand)]
#[command(rename = "lowercase", parse_with = "split")]
pub enum Command {
    #[command(rename = "/start")]
    Start,
    #[command(rename = "Помощь")]
    Help,
    #[command(rename = "/name")]
    Username(String),
    #[command(rename = "Имя и возраст")]
    UsernameAndAge {
        username: String,
        age: u8,
    },
    Unknown,
}

pub async fn answer(
    cx: UpdateWithCx<AutoSend<Bot>, Message>,
    command: Command,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    match command {
        Command::Start => {
            cx.answer("Вставай, самурай, и занимай очередь!")
                .reply_markup(general_keyboard())
                .await?
        }
        Command::Help => cx.answer("Help message").await?,
        Command::Username(full_name) => {
            let nickname = cx.update.from().unwrap().username.as_ref().unwrap();
            sqlx::query!(
                "INSERT INTO users(nickname, full_name) VALUES ($1, $2)\
                 ON CONFLICT (nickname) DO UPDATE SET full_name = $2",
                nickname,
                full_name
            )
            .execute(DATABASE.get().unwrap())
            .await?;
            cx.answer(format!("Теперь ты {}", full_name)).await?
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
