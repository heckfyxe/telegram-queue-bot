use crate::commands::Command;
use teloxide::types::{KeyboardButton, KeyboardMarkup, ReplyMarkup};

fn kb_markup(keyboard: Vec<Vec<String>>) -> ReplyMarkup {
    let kb: Vec<Vec<KeyboardButton>> = keyboard
        .iter()
        .map(|row| row.iter().map(|label| KeyboardButton::new(label)).collect())
        .collect();

    let markup = KeyboardMarkup::new(kb).resize_keyboard(true);

    ReplyMarkup::Keyboard(markup)
}

pub fn general_keyboard() -> ReplyMarkup {
    let commands = vec![
        Command::Help.to_string(),
        Command::Username.to_string(),
        Command::UsernameAndAge {
            username: "".to_string(),
            age: 0,
        }
        .to_string(),
    ];
    kb_markup(vec![commands])
}
