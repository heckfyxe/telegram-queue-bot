use teloxide::types::{KeyboardButton, KeyboardMarkup, ReplyMarkup};

fn kb_markup(keyboard: Vec<Vec<String>>) -> ReplyMarkup {
    let kb: Vec<Vec<KeyboardButton>> = keyboard
        .iter()
        .map(|row| row.iter().map(KeyboardButton::new).collect())
        .collect();

    let markup = KeyboardMarkup::new(kb).resize_keyboard(true);

    ReplyMarkup::Keyboard(markup)
}

pub fn general_keyboard() -> ReplyMarkup {
    let commands = vec!["Помощь".to_string(), "Имя и возраст".to_string()];
    kb_markup(vec![commands])
}
