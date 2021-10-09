mod commands;
mod webhook;

use crate::commands::answer;
use teloxide::prelude::*;
use teloxide::Bot;
use webhook::webhook;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    teloxide::enable_logging!();

    let bot = Bot::from_env().auto_send();

    teloxide::commands_repl_with_listener(
        bot.clone(),
        "Очередь 211-322",
        answer,
        webhook(bot).await,
    )
    .await;
}
