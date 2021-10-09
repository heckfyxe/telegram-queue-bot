use teloxide::{Bot, respond};
use teloxide::prelude::*;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    teloxide::enable_logging!();

    let bot = Bot::from_env().auto_send();

    teloxide::repl(bot, |message| async move {
        message.answer_dice().await?;
        respond(())
    }).await;
}
