use buongiornissimo_rs::{
    Augurando, BuongiornissimoCaffe, BuongiornoImmagini, Greeting, Scrape, TiCondivido,
};
use url::Url;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Providers {
    BuongiornissimoCaffe,
    BuongiornoImmagini,
    Augurando,
    TiCondivido,
}

impl Providers {
    /// Get all providers
    pub fn all() -> &'static [Providers] {
        &[
            Providers::BuongiornissimoCaffe,
            Providers::BuongiornoImmagini,
            Providers::Augurando,
            Providers::TiCondivido,
        ]
    }

    pub async fn scrape(self, greeting: Greeting) -> anyhow::Result<Vec<Url>> {
        let urls = match self {
            Providers::BuongiornissimoCaffe => BuongiornissimoCaffe.scrape(greeting).await,
            Providers::Augurando => Augurando.scrape(greeting).await,
            Providers::BuongiornoImmagini => BuongiornoImmagini.scrape(greeting).await,
            Providers::TiCondivido => TiCondivido.scrape(greeting).await,
        }?;

        Ok(urls)
    }
}
