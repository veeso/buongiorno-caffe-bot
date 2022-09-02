//! # Big luca bot repository
//!
//! This module contains the interface to the bot repository

use super::Config;
use crate::repository::{birthday::Birthday, chat::Chat, SqliteDb};

use chrono::NaiveDate;
use teloxide::types::ChatId;

pub struct Repository {
    db: SqliteDb,
}

impl Repository {
    /// Connect to the database
    pub async fn connect() -> anyhow::Result<Self> {
        let config = Config::try_from_env()
            .map_err(|_| anyhow::anyhow!("DATABASE_URL is not SET; repository is not available"))?;
        Ok(Self {
            db: SqliteDb::connect(&config.database_url)
                .await
                .map_err(|e| anyhow::anyhow!("failed to connect to the database: {}", e))?,
        })
    }

    /// Insert a chat to database
    pub async fn insert_chat(&self, chat: ChatId) -> anyhow::Result<()> {
        if self.is_subscribed(&chat).await? {
            anyhow::bail!(
                "Sei già iscritto a katanga. Se vuoi lucrare di più iscriviti ai miei corsi."
            )
        }
        Chat::new(chat)
            .insert(self.db.pool())
            .await
            .map_err(|e| anyhow::anyhow!("failed to insert chat into the database: {}", e))
    }

    /// Delete chat from database
    pub async fn delete_chat(&self, chat: ChatId) -> anyhow::Result<()> {
        Chat::new(chat)
            .delete(self.db.pool())
            .await
            .map_err(|e| anyhow::anyhow!("failed to delete chat from the database: {}", e))
    }

    /// Get subscribed chats
    pub async fn get_subscribed_chats(&self) -> anyhow::Result<Vec<ChatId>> {
        Chat::get_all(self.db.pool())
            .await
            .map_err(|e| anyhow::anyhow!("failed to collect subscribed chats: {}", e))
            .map(|x| {
                x.into_iter()
                    .map(|x| {
                        debug!(
                            "found subscribed chat {} ({})",
                            x.id(),
                            x.created_at()
                                .map(|x| x.to_rfc3339())
                                .unwrap_or_else(|_| String::from("date error"))
                        );
                        x.id()
                    })
                    .collect()
            })
    }

    /// Check whether `chat_id` is subscribed
    pub async fn is_subscribed(&self, chat_id: &ChatId) -> anyhow::Result<bool> {
        Ok(self
            .get_subscribed_chats()
            .await?
            .iter()
            .any(|x| x == chat_id))
    }

    // -- birthdays

    /// Insert a chat to database
    pub async fn insert_birthday(
        &self,
        chat: ChatId,
        name: String,
        date: NaiveDate,
    ) -> anyhow::Result<()> {
        if self.birthday_exists(&chat, &name, date).await? {
            anyhow::bail!("Questo compleanno è già presente registrato")
        }
        Birthday::new(chat, name, date)
            .insert(self.db.pool())
            .await
            .map_err(|e| anyhow::anyhow!("failed to insert birthdate into the database: {}", e))
    }

    /// Delete chat from database
    pub async fn delete_birthday_by_chat(&self, chat: ChatId) -> anyhow::Result<()> {
        Birthday::delete_by_chat(self.db.pool(), chat)
            .await
            .map_err(|e| anyhow::anyhow!("failed to delete birthday from the database: {}", e))
    }

    /// Get subscribed chats
    pub async fn get_birthdays(&self) -> anyhow::Result<Vec<(ChatId, String, NaiveDate)>> {
        Birthday::get_all(self.db.pool())
            .await
            .map_err(|e| anyhow::anyhow!("failed to collect birthdays: {}", e))
            .map(|x| {
                x.into_iter()
                    .map(|x| {
                        let date = x.date().unwrap();
                        debug!(
                            "found Birthday {} {} {} ({})",
                            x.chat(),
                            x.name(),
                            date,
                            x.created_at()
                                .map(|x| x.to_rfc3339())
                                .unwrap_or_else(|_| String::from("date error"))
                        );
                        (x.chat(), x.name().to_string(), date)
                    })
                    .collect()
            })
    }

    /// Check whether `chat_id` is subscribed
    async fn birthday_exists(
        &self,
        chat_id: &ChatId,
        name: &str,
        date: NaiveDate,
    ) -> anyhow::Result<bool> {
        Ok(self
            .get_birthdays()
            .await?
            .iter()
            .any(|(a, b, c)| a == chat_id && b == name && *c == date))
    }
}
