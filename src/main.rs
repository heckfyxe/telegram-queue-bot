mod webhook;

use teloxide::prelude::*;
use teloxide::{respond, Bot};
use webhook::webhook;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    teloxide::enable_logging!();

    let bot = Bot::from_env().auto_send();

    teloxide::repl_with_listener(
        bot.clone(),
        |message| async move {
            message.answer_dice().await?;
            respond(())
        },
        webhook(bot).await,
    )
    .await;
}
