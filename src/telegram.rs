use teloxide::{prelude::*, types::ChatId};
use std::error::Error;

pub async fn send_message_to_chat(chat_id_to_send_to: i64, text: String) -> Result<(), Box<dyn Error + Send + Sync>> {
    // Create a ChatId object from the raw integer ID
    let chat_id = ChatId(chat_id_to_send_to);

    // Send the message
    let bot = Bot::from_env();
    bot.send_message(chat_id, text).await?;

    println!("Message sent successfully to chat ID {}", chat_id_to_send_to);
    Ok(())
}