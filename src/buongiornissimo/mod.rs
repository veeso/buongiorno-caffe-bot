//! # Buongiornissimo
//!
//! This module exposes all the buongiornissimo providers

pub mod providers;

use async_trait::async_trait;
use url::Url;

/// Describes the Greeting type
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Greeting {
    BuonGiorno,
    BuonGiornoWeekday,
    BuonPomeriggio,
    BuonaNotte,
    Compleanno,
    FestaDellaRepubblica,
    Ferragosto,
    Halloween,
    /// Primo novembre
    Ognissanti,
    /// Refers to the 2nd of november
    Defunti,
    /// 8 dicembre
    ImmacolataConcenzione,
    VigiliaDiNatale,
    Natale,
}

#[async_trait]
pub trait Scrape {
    async fn scrape(&self, media: Greeting) -> anyhow::Result<Vec<Url>>;
}
