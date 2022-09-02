//! # Buongiornissimo
//!
//! This module exposes all the buongiornissimo providers

pub mod providers;

use async_trait::async_trait;
use url::Url;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Media {
    Auguri,
    BuonGiorno,
    BuonGiornoWeekday,
    BuonPomeriggio,
    BuonaNotte,
    BuonNatale,
}

#[async_trait]
pub trait Scrape {
    async fn scrape(&self, media: Media) -> anyhow::Result<Vec<Url>>;
}
