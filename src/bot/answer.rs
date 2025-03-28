//! # Answer
//!
//! This module cares of providing answer script types and sending messages

use teloxide::prelude::*;
use teloxide::types::InputFile;
use url::Url;

/// A helper to build composed answers
#[derive(Default)]
pub struct AnswerBuilder {
    answer: Answer,
}

impl AnswerBuilder {
    /// Add text to script
    pub fn text(mut self, text: impl ToString) -> Self {
        self.answer.script.push(Greeting::Text(text.to_string()));
        self
    }

    pub fn image(mut self, url: Url) -> Self {
        self.answer
            .script
            .push(Greeting::Image(InputFile::url(url)));
        self
    }

    /// Finalize builder
    pub fn finalize(self) -> Answer {
        self.answer
    }
}

/// The answer to send to the chat
#[derive(Default, Clone)]
pub struct Answer {
    script: Vec<Greeting>,
}

#[derive(Clone)]
/// A media in the chat
enum Greeting {
    Text(String),
    Image(InputFile),
}

impl Answer {
    /// Build a simple one text answer
    pub fn simple_text(text: impl ToString) -> Self {
        Self {
            script: vec![Greeting::Text(text.to_string())],
        }
    }

    /// Send answer
    pub async fn send(self, bot: &Bot, chat_id: ChatId) -> ResponseResult<()> {
        for message in self.script.into_iter() {
            match message {
                Greeting::Image(image) => Self::send_image(bot, chat_id, image).await?,
                Greeting::Text(text) => Self::send_text(bot, chat_id, text).await?,
            }
        }
        Ok(())
    }

    /// Write text to chat
    async fn send_text(bot: &Bot, chat_id: ChatId, message: String) -> ResponseResult<()> {
        bot.send_message(chat_id, message).await.map(|_| ())
    }

    /// Send image to chat
    async fn send_image(bot: &Bot, chat_id: ChatId, image: InputFile) -> ResponseResult<()> {
        bot.send_photo(chat_id, image).await.map(|_| ())
    }
}
