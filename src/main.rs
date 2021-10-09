use teloxide::prelude::*;
use teloxide::Bot;

use webhook::webhook;

use crate::commands::answer;

mod commands;
mod database;
mod keyboard;
mod webhook;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    teloxide::enable_logging!();
    database::initialize().await;

    let bot = Bot::from_env().auto_send();

    teloxide::repl_with_listener(bot.clone(), answer, webhook(bot).await).await;
}
