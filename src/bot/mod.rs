//! # Big Luca
//!
//! This module implements the big luca bot

mod answer;
mod automatize;
mod commands;
mod config;
mod redis;
mod repository;

use teloxide::{dispatching::update_listeners::webhooks, prelude::*, utils::command::BotCommands};

use answer::{Answer, AnswerBuilder};
use automatize::Automatizer;
use commands::Command;
pub use config::Config;
use once_cell::sync::OnceCell;
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
        let config = Config::try_from_env()?;
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
            Command::Caffeee => Self::subscribe_to_automatizer(&message.chat.id).await,
            Command::PuliziaKontatti => Self::unsubscribe_from_automatizer(&message.chat.id).await,
            Command::Release => Self::get_release(),
        };
        answer.send(&bot, message.chat.id).await
    }

    fn start() -> Answer {
        AnswerBuilder::default()
            .text("CAFFEE!? ☕ Entra subito nel mondo dei buongiornissimi con /cafeee o se vuoi un dolce assaggio dei miei contenuti /buongiornissimo altrimenti invia /help per vedere tutti i comandi disponibili")
            .finalize()
    }

    fn get_release() -> Answer {
        Answer::simple_text(format!(
            "buongiorno-caffe-bot ☕ {}. Sviluppato da @veeso97. Contribuisci al progetto su Github https://github.com/veeso/buongiorno-caffe-bot. Sostieni il mio progetto su Ko-Fi https://ko-fi.com/veeso",
            env!("CARGO_PKG_VERSION")
        ))
    }

    /// Subscribe chat to the automatizer
    async fn subscribe_to_automatizer(chat_id: &ChatId) -> Answer {
        match AUTOMATIZER.get().unwrap().subscribe(chat_id).await {
            Ok(_) => AnswerBuilder::default()
            .text("Buongiorno, CAFFEEE?! ☕☕☕  Da ora riceverei ogni giorno le migliori immagini di augurio.")
            .finalize(),
            Err(err) => Self::error(err),
        }
    }

    async fn unsubscribe_from_automatizer(chat_id: &ChatId) -> Answer {
        match AUTOMATIZER.get().unwrap().unsubscribe(chat_id).await {
            Ok(()) => AnswerBuilder::default()
                .text("ti sei disinscritto dai messaggi automatici ☕")
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
