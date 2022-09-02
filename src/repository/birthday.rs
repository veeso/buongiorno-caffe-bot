//! # Birthday
//!
//! this module contains the birthday entity repository

use std::str::FromStr;

use super::{RepositoryError, RepositoryResult};

use chrono::{DateTime, FixedOffset, NaiveDate, Utc};
use sqlx::{Pool, Sqlite};
use teloxide::types::ChatId;

#[derive(sqlx::FromRow, Debug, Clone, Eq, PartialEq)]
pub struct Birthday {
    chat: i64,
    name: String,
    date: String,
    created_at: String,
}

impl Birthday {
    pub fn new(chat_id: ChatId, name: String, date: NaiveDate) -> Self {
        Self {
            chat: chat_id.0,
            name,
            date: date.to_string(),
            created_at: Utc::now().to_rfc3339(),
        }
    }

    /// Return inner `ChatId`
    pub fn chat(&self) -> ChatId {
        ChatId(self.chat)
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    /// Return birthdate
    pub fn date(&self) -> RepositoryResult<NaiveDate> {
        NaiveDate::from_str(&self.date).map_err(|_| RepositoryError::BadDateTimeSyntax)
    }

    /// Return created_at as a `DateTime`
    pub fn created_at(&self) -> RepositoryResult<DateTime<FixedOffset>> {
        DateTime::parse_from_rfc3339(&self.created_at)
            .map_err(|_| RepositoryError::BadDateTimeSyntax)
    }

    /// Collect all the chat in the database
    pub async fn get_all(db: &Pool<Sqlite>) -> RepositoryResult<Vec<Birthday>> {
        sqlx::query_as(
            r#"
            SELECT chat, name, date, created_at
            FROM birthday"#,
        )
        .fetch_all(db)
        .await
        .map_err(RepositoryError::from)
    }

    /// Insert `Birthday` to database
    pub async fn insert(&self, db: &Pool<Sqlite>) -> RepositoryResult<()> {
        debug!("inserting a new chat {} to repository", self.chat);
        let rows = sqlx::query(
            "INSERT INTO birthday (chat, name, date, created_at) VALUES ($1, $2, $3, $4)",
        )
        .bind(self.chat)
        .bind(&self.name)
        .bind(&self.date)
        .bind(&self.created_at)
        .execute(db)
        .await
        .map_err(RepositoryError::from)?
        .rows_affected();
        if rows != 1 {
            return Err(RepositoryError::TooManyInserts);
        }

        Ok(())
    }

    /// Delete this chat from database
    pub async fn delete_by_chat(db: &Pool<Sqlite>, chat: ChatId) -> RepositoryResult<()> {
        debug!("deleting birthday for chat {} from repository", chat);
        sqlx::query("DELETE FROM birthday WHERE chat = $1")
            .bind(chat.0)
            .execute(db)
            .await
            .map_err(RepositoryError::from)?;

        Ok(())
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::repository::test::init_database;

    #[tokio::test]
    async fn should_insert_birthday() {
        let (db, temp) = init_database().await;
        let chat = Birthday::new(
            ChatId(32),
            "pippo".to_string(),
            NaiveDate::from_ymd(1997, 5, 30),
        );
        assert!(chat.insert(db.pool()).await.is_ok());
        drop(temp)
    }

    #[tokio::test]
    async fn should_delete_birthday() {
        let (db, temp) = init_database().await;
        let birthday = Birthday::new(
            ChatId(1),
            "pippo".to_string(),
            NaiveDate::from_ymd(1997, 5, 30),
        );
        assert!(birthday.insert(db.pool()).await.is_ok());
        assert!(Birthday::delete_by_chat(db.pool(), birthday.chat())
            .await
            .is_ok());
        drop(temp)
    }

    #[tokio::test]
    async fn should_retrieve_birthday() {
        let (db, temp) = init_database().await;
        let chats = vec![
            Birthday::new(
                ChatId(1),
                "pippo".to_string(),
                NaiveDate::from_ymd(1997, 5, 30),
            ),
            Birthday::new(
                ChatId(2),
                "pippo".to_string(),
                NaiveDate::from_ymd(1997, 5, 30),
            ),
            Birthday::new(
                ChatId(3),
                "pippo".to_string(),
                NaiveDate::from_ymd(1997, 5, 30),
            ),
        ];
        for chat in chats.iter() {
            assert!(chat.insert(db.pool()).await.is_ok());
        }
        // select
        assert_eq!(Birthday::get_all(db.pool()).await.unwrap(), chats);
        drop(temp)
    }
}
