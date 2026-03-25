//! # Automatize
//!
//! A module to automatize messages

use buongiornissimo_rs::Greeting;
use chrono::{Datelike, Local, NaiveDate};
use teloxide::prelude::*;
use teloxide::types::ChatId;
use thiserror::Error;
use tokio_cron_scheduler::{Job, JobScheduler, JobSchedulerError};
use tracing::{debug, error, info};

use super::AnswerBuilder;
use super::repository::Repository;
use crate::repository::SqliteDb;
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
    db: SqliteDb,
    bot: Bot,
    /// Stored to keep the scheduler alive for the lifetime of the application.
    #[expect(dead_code, reason = "held to keep the scheduler running")]
    scheduler: JobScheduler,
}

impl Automatizer {
    /// Start automatizer with the given database and bot
    pub async fn start(db: SqliteDb, bot: Bot) -> AutomatizerResult<Self> {
        debug!("starting automatizer");
        Ok(Self {
            db,
            bot,
            scheduler: Self::setup_cron_scheduler().await?,
        })
    }

    /// Subscribe a chat to the automatizer
    pub async fn subscribe(&self, chat: &ChatId) -> anyhow::Result<()> {
        let repository = self.repository();
        repository.insert_chat(*chat).await?;
        info!("subscribed {} to the automatizer", chat);
        Ok(())
    }

    /// Unsubscribe chat from automatizer. If the chat is not currently subscribed, return error
    pub async fn unsubscribe(&self, chat: &ChatId) -> anyhow::Result<()> {
        let repository = self.repository();
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
        let repository = self.repository();
        repository
            .insert_birthday(*chat, name.clone(), date)
            .await?;
        info!(
            "registered birthday for {}, name {}, date: {}",
            chat, name, date
        );
        Ok(())
    }

    /// Get subscribed chats
    pub async fn subscribed_chats(&self) -> anyhow::Result<Vec<ChatId>> {
        self.repository().get_subscribed_chats().await
    }

    /// Send happy birthday greetings for today's birthdays
    pub async fn send_happy_birthday(&self) -> anyhow::Result<()> {
        let today_birthdays = self.today_birthdays().await?;
        if today_birthdays.is_empty() {
            return Ok(());
        }
        let image = super::Buongiornissimo::get_greeting_image(Greeting::Compleanno).await?;
        for (chat, name, _) in today_birthdays.into_iter() {
            if let Err(err) = AnswerBuilder::default()
                .image(image.clone())
                .text(format!("Buon compleanno {}!", name))
                .finalize()
                .send(&self.bot, chat)
                .await
            {
                error!("failed to send happy birthday to {}: {}", chat, err);
            }
        }
        Ok(())
    }

    /// Send good morning greeting
    pub async fn send_good_morning(&self) -> anyhow::Result<()> {
        self.send_greeting(buongiornissimo_rs::greeting_of_the_day(
            Local::now().naive_local().into(),
            *random_utils::choice(&[true, false]),
        ))
        .await
    }

    /// Send generic greeting to all subscribed chats
    pub async fn send_greeting(&self, media: Greeting) -> anyhow::Result<()> {
        let subscribed_chats = self.subscribed_chats().await?;
        if subscribed_chats.is_empty() {
            return Ok(());
        }
        let greeting = super::Buongiornissimo::get_greeting_image(media).await?;
        let answer = AnswerBuilder::default().image(greeting).finalize();
        for chat in subscribed_chats.iter() {
            if let Err(err) = answer.clone().send(&self.bot, *chat).await {
                error!("failed to send scheduled greeting to {}: {}", chat, err);
            }
        }
        Ok(())
    }

    fn repository(&self) -> Repository {
        Repository::new(self.db.clone())
    }

    /// Retrieve today's birthdays
    async fn today_birthdays(&self) -> anyhow::Result<Vec<(ChatId, String, NaiveDate)>> {
        let today = Local::now().naive_local();
        Ok(self
            .repository()
            .get_birthdays()
            .await?
            .into_iter()
            .filter(|(_, _, date)| date.month() == today.month() && date.day() == today.day())
            .collect())
    }

    /// Setup cron scheduler
    async fn setup_cron_scheduler() -> AutomatizerResult<JobScheduler> {
        let timezone = chrono::Local::now().timezone();
        let sched = JobScheduler::new().await?;

        let jobs: &[(&str, &str)] = &[
            ("0 30 8 * * *", "happy_birthday"),
            ("0 30 6 * * *", "good_morning"),
            ("0 15 20 * * Fri", "good_weekend"),
            ("0 30 12 * * *", "good_lunch"),
            ("0 0 14 * * *", "good_afternoon"),
            ("0 0 18 * * *", "good_evening"),
            ("0 30 19 * * *", "good_dinner"),
            ("0 30 21 * * *", "good_night"),
        ];

        for &(cron_expr, job_name) in jobs {
            let name = job_name.to_string();
            let job = Job::new_async_tz(cron_expr, timezone, move |_, _| {
                let name = name.clone();
                Box::pin(async move {
                    info!("running {name}_job");
                    if let Err(err) = Self::run_scheduled_job(&name).await {
                        error!("{name}_job failed: {}", err);
                    }
                })
            })?;
            sched.add(job).await?;
        }

        sched
            .start()
            .await
            .map(|_| sched)
            .map_err(AutomatizerError::from)
    }

    /// Run a scheduled job by name, dispatching to the AUTOMATIZER static
    async fn run_scheduled_job(name: &str) -> anyhow::Result<()> {
        let automatizer = super::AUTOMATIZER
            .get()
            .ok_or_else(|| anyhow::anyhow!("automatizer not initialized"))?;

        match name {
            "happy_birthday" => automatizer.send_happy_birthday().await,
            "good_morning" => automatizer.send_good_morning().await,
            "good_weekend" => automatizer.send_greeting(Greeting::Weekend).await,
            "good_lunch" => automatizer.send_greeting(Greeting::BuonPranzo).await,
            "good_afternoon" => automatizer.send_greeting(Greeting::BuonPomeriggio).await,
            "good_evening" => automatizer.send_greeting(Greeting::BuonaSerata).await,
            "good_dinner" => automatizer.send_greeting(Greeting::BuonaCena).await,
            "good_night" => automatizer.send_greeting(Greeting::BuonaNotte).await,
            _ => anyhow::bail!("unknown job: {name}"),
        }
    }
}
