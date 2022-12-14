//! # Big Luca
//!
//! This module implements the big luca bot

mod answer;
mod automatize;
mod commands;
mod config;
mod repository;

use crate::utils::random as random_utils;

use answer::{Answer, AnswerBuilder};
use automatize::Automatizer;
use buongiornissimo_rs::{BuongiornissimoCaffe, Greeting, IlMondoDiGrazia, Scrape, ScrapeError};
use chrono::{Local, NaiveDate};
use commands::Command;
pub use config::Config;
use once_cell::sync::OnceCell;
use teloxide::{dispatching::update_listeners::webhooks, prelude::*, utils::command::BotCommands};
use url::Url;

pub static AUTOMATIZER: OnceCell<Automatizer> = OnceCell::new();

/// Big luca bot application
pub struct Buongiornissimo {
    bot: AutoSend<Bot>,
}

impl Buongiornissimo {
    /// Initialize big luca
    pub async fn init() -> anyhow::Result<Self> {
        // parse configuration
        if let Err(err) = Config::try_from_env() {
            return Err(err);
        }
        let automatizer = Automatizer::start()
            .await
            .map_err(|e| anyhow::anyhow!("failed to start automatizer: {}", e))?;
        // read parameters
        if AUTOMATIZER.set(automatizer).is_err() {
            anyhow::bail!("failed to set automatizer");
        };
        let bot = Bot::from_env().auto_send();
        Ok(Self { bot })
    }

    /// Run big luca bot
    pub async fn run(self) -> anyhow::Result<()> {
        // setup hooks
        let port = Self::get_heroku_port()?;
        if let Some(port) = port {
            Self::run_on_heroku(self, port).await
        } else {
            Self::run_simple(self).await
        }
    }

    /// run bot with heroku webhooks
    async fn run_on_heroku(self, port: u16) -> anyhow::Result<()> {
        info!("running bot with heroku listener (PORT: {})", port);
        let addr = ([0, 0, 0, 0], port).into();
        let token = self.bot.inner().token();
        let host = std::env::var("HOST").map_err(|_| anyhow::anyhow!("HOST is not SET"))?;
        let url = Url::parse(&format!("https://{host}/webhooks/{token}")).unwrap();
        debug!("configuring listener {}...", url);
        let listener = webhooks::axum(self.bot.clone(), webhooks::Options::new(addr, url))
            .await
            .map_err(|e| anyhow::anyhow!("could not configure listener: {}", e))?;
        // start bot
        teloxide::commands_repl_with_listener(self.bot, Self::answer, listener, Command::ty())
            .await;
        Ok(())
    }

    /// run bot without webhooks
    async fn run_simple(self) -> anyhow::Result<()> {
        info!("running bot without webhooks");
        teloxide::commands_repl(self.bot, Self::answer, Command::ty()).await;
        Ok(())
    }

    /// Answer handler for bot
    async fn answer(
        bot: AutoSend<Bot>,
        message: Message,
        command: Command,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        debug!("got command {:?}", command);
        let answer = match command {
            Command::Help => Answer::simple_text(Command::descriptions()),
            Command::Start => Self::start(),
            Command::Auguri { name } => Self::happy_birthday(&name).await,
            Command::Buongiornissimo => Self::get_buongiornissimo_buongiorno().await,
            Command::Buonanotte => Self::get_buongiornissimo(Greeting::BuonaNotte).await,
            Command::Buonpomeriggio => Self::get_buongiornissimo(Greeting::BuonPomeriggio).await,
            Command::Compleanno { name, date } => {
                Self::subscribe_birthday(&message.chat.id, name, date).await
            }
            Command::Caffeee => Self::subscribe_to_automatizer(&message.chat.id).await,
            Command::PuliziaKontatti => Self::unsubscribe_from_automatizer(&message.chat.id).await,
            Command::Release => Self::get_release(),
        };
        answer.send(&bot, message.chat.id).await
    }

    fn start() -> Answer {
        AnswerBuilder::default()
            .text("CAFFEE!? ??? Entra subito nel mondo dei buongiornissimi con /caffeee o se vuoi un dolce assaggio dei miei contenuti /buongiornissimo altrimenti invia /help per vedere tutti i comandi disponibili")
            .finalize()
    }

    fn get_release() -> Answer {
        Answer::simple_text(format!(
            "buongiorno-caffe-bot ??? {}. Sviluppato da @veeso97. Contribuisci al progetto su Github https://github.com/veeso/buongiorno-caffe-bot. Sostieni il mio progetto su Ko-Fi https://ko-fi.com/veeso",
            env!("CARGO_PKG_VERSION")
        ))
    }

    /// Get a buongiorno only image
    pub async fn get_buongiornissimo_buongiorno() -> Answer {
        Self::get_buongiornissimo(buongiornissimo_rs::greeting_of_the_day(
            Local::today().naive_local(),
            *random_utils::choice(&[true, false]),
        ))
        .await
    }

    /// Get buongiornissimo for media type
    pub async fn get_buongiornissimo(media: Greeting) -> Answer {
        match Self::get_greeting_image(media).await {
            Ok(image) => AnswerBuilder::default().image(image).finalize(),
            Err(err) => Self::error(err),
        }
    }

    /// Get happy birthday answer
    pub async fn happy_birthday(name: &str) -> Answer {
        let image = match Self::get_greeting_image(Greeting::Compleanno).await {
            Ok(url) => url,
            Err(err) => return Self::error(err),
        };
        AnswerBuilder::default()
            .image(image)
            .text(format!("Buon compleanno {}!", name))
            .finalize()
    }

    /// Get greeting image for media type.
    /// At the first try it'll use a random provider; then if the media type is not supported, it tries all the different providers
    pub async fn get_greeting_image(media: Greeting) -> anyhow::Result<Url> {
        match random_utils::choice(&[0, 1]) {
            0 => Self::do_get_greeting_image(IlMondoDiGrazia::default(), media).await,
            _ => Self::do_get_greeting_image(BuongiornissimoCaffe::default(), media).await,
        }
    }

    async fn do_get_greeting_image(
        provider: impl Scrape,
        greeting: Greeting,
    ) -> anyhow::Result<Url> {
        match provider.scrape(greeting).await {
            Ok(urls) => Ok(random_utils::choice(&urls).clone()),
            Err(ScrapeError::UnsupportedGreeting) => Self::try_all_providers(greeting).await,
            Err(err) => anyhow::bail!("failed to scrape image: {}", err),
        }
    }

    /// Try all previders for media
    async fn try_all_providers(media: Greeting) -> anyhow::Result<Url> {
        if let Ok(urls) = IlMondoDiGrazia::default().scrape(media).await {
            return Ok(random_utils::choice(&urls).clone());
        }
        if let Ok(urls) = BuongiornissimoCaffe::default().scrape(media).await {
            return Ok(random_utils::choice(&urls).clone());
        }

        // try to get buongiorno
        match random_utils::choice(&[0, 1]) {
            0 => IlMondoDiGrazia::default()
                .scrape(Greeting::BuonGiorno)
                .await
                .map(|urls| random_utils::choice(&urls).clone())
                .map_err(|e| anyhow::anyhow!("failed to scrape image: {}", e)),
            _ => BuongiornissimoCaffe::default()
                .scrape(Greeting::BuonGiorno)
                .await
                .map(|urls| random_utils::choice(&urls).clone())
                .map_err(|e| anyhow::anyhow!("failed to scrape image: {}", e)),
        }
    }

    /// Subscribe birthday
    async fn subscribe_birthday(chat_id: &ChatId, name: String, date: NaiveDate) -> Answer {
        match AUTOMATIZER.get().unwrap().add_birthday(chat_id, name.clone(), date).await {
            Ok(_) => AnswerBuilder::default()
            .text(format!("Buongiorno, CAFFEEE?! ?????????  Da ora {} ricever?? gli auguri il giorno del suo compleanno.", name))
            .finalize(),
            Err(err) => Self::error(err),
        }
    }

    /// Subscribe chat to the automatizer
    async fn subscribe_to_automatizer(chat_id: &ChatId) -> Answer {
        match AUTOMATIZER.get().unwrap().subscribe(chat_id).await {
            Ok(_) => AnswerBuilder::default()
            .text("Buongiorno, CAFFEEE?! ?????????  Da ora riceverei ogni giorno le migliori immagini di augurio.")
            .finalize(),
            Err(err) => Self::error(err),
        }
    }

    async fn unsubscribe_from_automatizer(chat_id: &ChatId) -> Answer {
        match AUTOMATIZER.get().unwrap().unsubscribe(chat_id).await {
            Ok(()) => AnswerBuilder::default()
                .text("ti sei disinscritto dai messaggi automatici ???")
                .finalize(),
            Err(err) => Self::error(err),
        }
    }

    /// The answer to return in case of an error
    fn error(err: impl ToString) -> Answer {
        AnswerBuilder::default().text(err).finalize()
    }

    // get heroku port
    fn get_heroku_port() -> anyhow::Result<Option<u16>> {
        match std::env::var("PORT").map(|x| x.parse()) {
            Err(_) => Ok(None),
            Ok(Ok(p)) => Ok(Some(p)),
            Ok(Err(e)) => anyhow::bail!("could not parse PORT environment variable: {}", e),
        }
    }
}
