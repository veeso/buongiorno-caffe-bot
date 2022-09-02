//! # Automatize
//!
//! A module to automatize messages

use crate::buongiornissimo::Media;

use super::repository::Repository;
use super::AnswerBuilder;
use crate::utils::random as random_utils;

use chrono::{Datelike, Local, NaiveDate};
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
        // birthday job
        let happy_birthday_job = Job::new_async("0 30 8 * * *", |_, _| {
            Box::pin(async move {
                info!("running morning_job");
                if let Err(err) = Self::send_happy_birthday().await {
                    error!("happy_birthday_job failed: {}", err);
                }
            })
        })?;
        sched.add(happy_birthday_job).await?;
        // aphorism jobs
        let good_morning_job = Job::new_async("0 30 6 * * *", |_, _| {
            Box::pin(async move {
                info!("running morning_job");
                if let Err(err) = Self::send_good_morning().await {
                    error!("good_morning_job failed: {}", err);
                }
            })
        })?;
        sched.add(good_morning_job).await?;

        let good_afternoon_job = Job::new_async("0 40 12 * * *", |_, _| {
            Box::pin(async move {
                info!("running afternoon_job");
                if let Err(err) = Self::send_greeting(Media::BuonPomeriggio).await {
                    error!("good_afternoon_job failed: {}", err);
                }
            })
        })?;
        sched.add(good_afternoon_job).await?;

        let good_night_job = Job::new_async("0 40 12 * * *", |_, _| {
            Box::pin(async move {
                info!("running afternoon_job");
                if let Err(err) = Self::send_greeting(Media::BuonaNotte).await {
                    error!("good_night_job failed: {}", err);
                }
            })
        })?;
        sched.add(good_night_job).await?;

        sched
            .start()
            .await
            .map(|_| sched)
            .map_err(AutomatizerError::from)
    }

    /// Send happy birthday
    async fn send_happy_birthday() -> anyhow::Result<()> {
        let today_birthdays = Self::today_birthdays().await?;
        if today_birthdays.is_empty() {
            return Ok(());
        }
        let image = super::Buongiornissimo::get_buongiornissimo_image(Media::Auguri).await?;
        let bot = Bot::from_env().auto_send();
        for (chat, name, _) in today_birthdays.into_iter() {
            if let Err(err) = AnswerBuilder::default()
                .image(image.clone())
                .text(format!("Buon compleanno {}!", name))
                .finalize()
                .send(&bot, chat)
                .await
            {
                error!("failed to send happy birthday to {}: {}", chat, err);
            }
        }
        Ok(())
    }

    /// Send good morning greeting
    async fn send_good_morning() -> anyhow::Result<()> {
        let media = match Local::today().naive_local() {
            date if date == NaiveDate::from_ymd(date.year(), 12, 25) => Media::BuonNatale,
            _ => *random_utils::choice(&[Media::BuonGiorno, Media::BuonGiornoWeekday]),
        };
        Self::send_greeting(media).await
    }

    /// Send generic greeting
    async fn send_greeting(media: Media) -> anyhow::Result<()> {
        let subscribed_chats = Self::subscribed_chats().await?;
        if subscribed_chats.is_empty() {
            return Ok(());
        }
        let greeting = super::Buongiornissimo::get_buongiornissimo_image(media).await?;
        let answer = AnswerBuilder::default().image(greeting).finalize();
        let bot = Bot::from_env().auto_send();
        for chat in subscribed_chats.iter() {
            if let Err(err) = answer.clone().send(&bot, *chat).await {
                error!("failed to send scheduled greeting to {}: {}", chat, err);
            }
        }
        Ok(())
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
