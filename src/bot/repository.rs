//! # Bot repository
//!
//! This module contains the interface to the bot repository

use chrono::NaiveDate;
use teloxide::types::ChatId;
use tracing::debug;

use crate::repository::SqliteDb;
use crate::repository::birthday::Birthday;
use crate::repository::chat::Chat;

pub struct Repository {
    db: SqliteDb,
}

impl Repository {
    /// Create a new repository with the given database connection
    pub fn new(db: SqliteDb) -> Self {
        Self { db }
    }

    /// Insert a chat to database
    pub async fn insert_chat(&self, chat: ChatId) -> anyhow::Result<()> {
        if self.is_subscribed(&chat).await? {
            anyhow::bail!("Sei già iscritto ai messaggi automatici.")
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
        Chat::exists(self.db.pool(), chat_id.0)
            .await
            .map_err(|e| anyhow::anyhow!("failed to check subscription: {}", e))
    }

    // -- birthdays

    /// Insert a birthday to database
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

    /// Delete birthdays by chat from database
    pub async fn delete_birthday_by_chat(&self, chat: ChatId) -> anyhow::Result<()> {
        Birthday::delete_by_chat(self.db.pool(), chat)
            .await
            .map_err(|e| anyhow::anyhow!("failed to delete birthday from the database: {}", e))
    }

    /// Get all birthdays
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

    /// Check whether a birthday exists
    async fn birthday_exists(
        &self,
        chat_id: &ChatId,
        name: &str,
        date: NaiveDate,
    ) -> anyhow::Result<bool> {
        Birthday::exists(self.db.pool(), chat_id.0, name, &date.to_string())
            .await
            .map_err(|e| anyhow::anyhow!("failed to check birthday existence: {}", e))
    }
}

#[cfg(test)]
mod test {

    use pretty_assertions::assert_eq;
    use tempfile::NamedTempFile;

    use super::*;
    use crate::repository::SqliteDb;

    async fn setup_repository() -> (Repository, NamedTempFile) {
        let database = NamedTempFile::new().unwrap();
        let db = SqliteDb::connect(&database.path().to_string_lossy())
            .await
            .unwrap();
        (Repository::new(db), database)
    }

    #[tokio::test]
    async fn should_handle_chats() {
        let (repository, _database) = setup_repository().await;
        assert!(repository.insert_chat(ChatId(1)).await.is_ok());
        assert!(repository.insert_chat(ChatId(2)).await.is_ok());
        // duped
        assert!(repository.insert_chat(ChatId(1)).await.is_err());
        // get chats
        assert_eq!(repository.get_subscribed_chats().await.unwrap().len(), 2);
        // delete
        assert!(repository.delete_chat(ChatId(2)).await.is_ok());
    }

    #[tokio::test]
    async fn should_handle_birthdays() {
        let (repository, _database) = setup_repository().await;
        assert!(
            repository
                .insert_birthday(
                    ChatId(1),
                    "Christian".to_string(),
                    NaiveDate::from_ymd_opt(1997, 5, 30).unwrap()
                )
                .await
                .is_ok()
        );
        assert!(
            repository
                .insert_birthday(
                    ChatId(1),
                    "Chiara".to_string(),
                    NaiveDate::from_ymd_opt(1999, 6, 24).unwrap()
                )
                .await
                .is_ok()
        );
        // duped
        assert!(
            repository
                .insert_birthday(
                    ChatId(1),
                    "Chiara".to_string(),
                    NaiveDate::from_ymd_opt(1999, 6, 24).unwrap()
                )
                .await
                .is_err()
        );
        // get birthdays
        assert_eq!(repository.get_birthdays().await.unwrap().len(), 2);
        // delete
        assert!(repository.delete_birthday_by_chat(ChatId(1)).await.is_ok());
        assert!(repository.get_birthdays().await.unwrap().is_empty());
    }
}
