//! # Automatize
//!
//! A module to automatize messages

use buongiornissimo_rs::Greeting;
use chrono::{Datelike, Local, NaiveDate};
use teloxide::prelude::*;
use teloxide::types::ChatId;
use thiserror::Error;
use tokio_cron_scheduler::{Job, JobScheduler, JobSchedulerError};

use super::AnswerBuilder;
use super::repository::Repository;
use crate::utils::random as random_utils;

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
            anyhow::bail!(
                "devi prima sottoscriverti ai messaggi automatici, prima di configurare un compleanno. Iscriviti con /caffeee"
            );
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
                info!("running happy_birthday_job");
                if let Err(err) = Self::send_happy_birthday().await {
                    error!("happy_birthday_job failed: {}", err);
                }
            })
        })?;
        sched.add(happy_birthday_job).await?;

        // good morning
        let good_morning_job = Job::new_async("0 30 6 * * *", |_, _| {
            Box::pin(async move {
                info!("running good_morning_job");
                if let Err(err) = Self::send_good_morning().await {
                    error!("good_morning_job failed: {}", err);
                }
            })
        })?;
        sched.add(good_morning_job).await?;

        // buon pranzo
        let good_lunch_job = Job::new_async("0 30 12 * * *", |_, _| {
            Box::pin(async move {
                info!("running good_lunch_job");
                if let Err(err) = Self::send_greeting(Greeting::BuonPranzo).await {
                    error!("good_lunch_job failed: {}", err);
                }
            })
        })?;
        sched.add(good_lunch_job).await?;

        // buon pomeriggio
        let good_afternoon_job = Job::new_async("0 40 12 * * *", |_, _| {
            Box::pin(async move {
                info!("running good_afternoon_job");
                if let Err(err) = Self::send_greeting(Greeting::BuonPomeriggio).await {
                    error!("good_afternoon_job failed: {}", err);
                }
            })
        })?;
        sched.add(good_afternoon_job).await?;

        // buona serata
        let good_evening_job = Job::new_async("0 30 18 * * *", |_, _| {
            Box::pin(async move {
                info!("running good_evening_job");
                if let Err(err) = Self::send_greeting(Greeting::BuonaSerata).await {
                    error!("good_evening_job failed: {}", err);
                }
            })
        })?;
        sched.add(good_evening_job).await?;

        // buona cena
        let good_dinner_job = Job::new_async("0 0 20 * * *", |_, _| {
            Box::pin(async move {
                info!("running good_dinner_job");
                if let Err(err) = Self::send_greeting(Greeting::BuonaCena).await {
                    error!("good_dinner_job failed: {}", err);
                }
            })
        })?;
        sched.add(good_dinner_job).await?;

        // buona notte
        let good_night_job = Job::new_async("0 30 21 * * *", |_, _| {
            Box::pin(async move {
                info!("running good_night_job");
                if let Err(err) = Self::send_greeting(Greeting::BuonaNotte).await {
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
        let image = super::Buongiornissimo::get_greeting_image(Greeting::Compleanno).await?;
        let bot = Bot::from_env();
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
        Self::send_greeting(buongiornissimo_rs::greeting_of_the_day(
            Local::now().naive_local().into(),
            *random_utils::choice(&[true, false]),
        ))
        .await
    }

    /// Send generic greeting
    async fn send_greeting(media: Greeting) -> anyhow::Result<()> {
        let subscribed_chats = Self::subscribed_chats().await?;
        if subscribed_chats.is_empty() {
            return Ok(());
        }
        let greeting = super::Buongiornissimo::get_greeting_image(media).await?;
        let answer = AnswerBuilder::default().image(greeting).finalize();
        let bot = Bot::from_env();
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
        let today = Local::now().naive_local();
        Ok(repository
            .get_birthdays()
            .await?
            .into_iter()
            .filter(|(_, _, date)| date.month() == today.month() && date.day() == today.day())
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
