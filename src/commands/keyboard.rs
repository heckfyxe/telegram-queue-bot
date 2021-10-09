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
    let commands = vec![String::from("/username"), String::from("/usernameandage")];
    kb_markup(vec![commands])
}
