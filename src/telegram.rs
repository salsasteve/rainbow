use teloxide::{prelude::*, types::ChatId};
use std::error::Error;

/// Function to send a message to a specific chat ID using the Telegram Bot API.
pub async fn send_message_to_chat(chat_id_to_send_to: i64, text: String) -> Result<(), Box<dyn Error + Send + Sync>> {
    let chat_id = ChatId(chat_id_to_send_to);

    let bot = Bot::from_env();
    bot.send_message(chat_id, text).await?;

    println!("Message sent successfully to chat ID {}", chat_id_to_send_to);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_send_message_to_chat() {
        let chat_id = 123456789; // Replace with a valid chat ID
        let message = "Hello, this is a test message!".to_string();

        let result = send_message_to_chat(chat_id, message).await;
        assert!(result.is_ok());
    }
}