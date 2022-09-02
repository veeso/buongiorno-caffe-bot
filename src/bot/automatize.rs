//! # Automatize
//!
//! A module to automatize messages

use super::repository::Repository;
use super::AnswerBuilder;

use chrono::{Local, NaiveDate};
use teloxide::prelude::*;
use teloxide::types::ChatId;
use thiserror::Error;
use tokio_cron_scheduler::{Job, JobScheduler, JobSchedulerError};

type AutomatizerResult<T> = Result<T, AutomatizerError>;

/// Automatizer error
#[derive(Debug, Error)]
pub enum AutomatizerError {
    #[error("scheduler error: {0}")]
    Scheduler(JobSchedulerError),
}

impl From<JobSchedulerError> for AutomatizerError {
    fn from(e: JobSchedulerError) -> Self {
        Self::Scheduler(e)
    }
}

/// Automatizer takes care of sending messages to subscribed users
pub struct Automatizer {
    scheduler: JobScheduler,
}

impl Automatizer {
    /// Start automatizer
    pub async fn start() -> AutomatizerResult<Self> {
        debug!("starting automatizer");
        Ok(Self {
            scheduler: Self::setup_cron_scheduler().await?,
        })
    }

    /// Subscribe a chat to the automatizer
    pub async fn subscribe(&self, chat: &ChatId) -> anyhow::Result<()> {
        let repository = Repository::connect().await?;
        repository.insert_chat(*chat).await?;
        info!("subscribed {} to the automatizer", chat);
        Ok(())
    }

    /// Unsubscribe chat from automatizer. If the chat is not currently subscribed, return error
    pub async fn unsubscribe(&self, chat: &ChatId) -> anyhow::Result<()> {
        let repository = Repository::connect().await?;
        repository.delete_birthday_by_chat(*chat).await?;
        info!("deleted birthdays associated to chat {}", chat);
        repository.delete_chat(*chat).await?;
        info!("unsubscribed {} from the automatizer", chat);
        Ok(())
    }

    /// Add birthday to repository
    pub async fn add_birthday(
        &self,
        chat: &ChatId,
        name: String,
        date: NaiveDate,
    ) -> anyhow::Result<()> {
        let repository = Repository::connect().await?;
        // check whether chat is subscribed to automatic message
        if !repository.is_subscribed(chat).await? {
            anyhow::bail!("devi prima sottoscriverti ai messaggi automatici, prima di configurare un compleanno. Iscriviti con /caffeee");
        }
        repository
            .insert_birthday(*chat, name.clone(), date)
            .await?;
        info!(
            "registered birthday for {}, name {}, date: {}",
            chat, name, date
        );
        Ok(())
    }

    /// Setup cron scheduler
    async fn setup_cron_scheduler() -> AutomatizerResult<JobScheduler> {
        let sched = JobScheduler::new().await?;
        // aphorism jobs
        /*let morning_aphorism_job = Job::new_async("0 0 6 * * *", |_, _| {
            Box::pin(async move {
                info!("running morning_aphorism_job");
                if let Err(err) = Self::send_perla().await {
                    error!("evening_aphorism_job failed: {}", err);
                }
            })
            sched.add(morning_aphorism_job).await?;
        })?;*/

        sched
            .start()
            .await
            .map(|_| sched)
            .map_err(AutomatizerError::from)
    }

    pub async fn subscribed_chats() -> anyhow::Result<Vec<ChatId>> {
        let repository = Repository::connect().await?;
        repository.get_subscribed_chats().await
    }

    /// Retrieve today birthdays
    async fn today_birthdays() -> anyhow::Result<Vec<(ChatId, String, NaiveDate)>> {
        let repository = Repository::connect().await?;
        let today = Local::today().naive_local();
        Ok(repository
            .get_birthdays()
            .await?
            .into_iter()
            .filter(|(_, _, date)| *date == today)
            .collect())
    }
}

impl Drop for Automatizer {
    fn drop(&mut self) {
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            info!("Shutting scheduler down");
            if let Err(err) = self.scheduler.shutdown().await {
                error!("failed to stop scheduler: {}", err);
            }
        });
    }
}
