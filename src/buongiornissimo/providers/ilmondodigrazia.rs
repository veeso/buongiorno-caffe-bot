//! # Il mondo di grazia
//!
//! This provider provides images from <https://ilmondodigrazia.com>

use const_format::concatcp;
use std::str::FromStr;

use super::{Greeting, Scrape, Url};

use async_trait::async_trait;
use chrono::{Datelike, Local, Weekday};
use scraper::{Html, Selector};

const BASE_URL: &str = "https://ilmondodigrazia.com";
const AUGURI_URL: &str = concatcp!(BASE_URL, "/compleanno");
const BUONGIORNO_URL: &str = concatcp!(BASE_URL, "buongiorno");
const BUONGIORNO_WEEKDAY_BASE_URL: &str = concatcp!(BASE_URL, "/buongiorno/buongiorno-");
const BUON_POMERIGGIO_URL: &str = concatcp!(BASE_URL, "/buon-pomeriggio");
const BUONA_NOTTE_URL: &str = concatcp!(BASE_URL, "/buonanotte");
const DUE_GIUGNO_URL: &str = concatcp!(BASE_URL, "/festa/festa-della-repubblica");
const FERRAGOSTO_URL: &str = concatcp!(BASE_URL, "/buon-ferragosto");
const OGNISSANTI_URL: &str = concatcp!(BASE_URL, "/tutti-i-santi-immagini-festa-di-ognissanti");
const DUE_NOVEMBRE_URL: &str = concatcp!(BASE_URL, "/commemorazione-dei-defunti-2-novembre");
const HALLOWEEN_URL: &str = concatcp!(BASE_URL, "/halloween-31-ottobre-immagini-buongiorno");
const IMMACOLATA_CONCEZIONE_URL: &str =
    concatcp!(BASE_URL, "/immacolata-concezione-8-dicembre-buongiorno");
const VIGILIA_URL: &str = concatcp!(
    BASE_URL,
    "/buongiorno/buongiorno-vigilia-di-natale-per-il-24-dicembre"
);
const BUON_NATALE_URL: &str = concatcp!(BASE_URL, "/frasi-di-buon-natale-immagini-da-condividere");

#[derive(Default)]
pub struct IlMondoDiGrazia;

impl IlMondoDiGrazia {
    fn weekday() -> &'static str {
        match Local::today().weekday() {
            Weekday::Mon => "lunedi",
            Weekday::Tue => "martedi",
            Weekday::Wed => "mercoledi",
            Weekday::Thu => "giovedi",
            Weekday::Fri => "venerdi",
            Weekday::Sat => "sabato",
            Weekday::Sun => "domenica",
        }
    }

    fn buongiorno_weekday_url() -> String {
        format!("{}{}", BUONGIORNO_WEEKDAY_BASE_URL, Self::weekday())
    }

    fn get_url(media: Greeting) -> String {
        match media {
            Greeting::Compleanno => AUGURI_URL.to_string(),
            Greeting::BuonGiorno => BUONGIORNO_URL.to_string(),
            Greeting::BuonGiornoWeekday => Self::buongiorno_weekday_url(),
            Greeting::BuonPomeriggio => BUON_POMERIGGIO_URL.to_string(),
            Greeting::BuonaNotte => BUONA_NOTTE_URL.to_string(),
            Greeting::FestaDellaRepubblica => DUE_GIUGNO_URL.to_string(),
            Greeting::Ferragosto => FERRAGOSTO_URL.to_string(),
            Greeting::Ognissanti => OGNISSANTI_URL.to_string(),
            Greeting::Defunti => DUE_NOVEMBRE_URL.to_string(),
            Greeting::Halloween => HALLOWEEN_URL.to_string(),
            Greeting::ImmacolataConcenzione => IMMACOLATA_CONCEZIONE_URL.to_string(),
            Greeting::VigiliaDiNatale => VIGILIA_URL.to_string(),
            Greeting::Natale => BUON_NATALE_URL.to_string(),
        }
    }
}

#[async_trait]
impl Scrape for IlMondoDiGrazia {
    async fn scrape(&self, media: Greeting) -> anyhow::Result<Vec<Url>> {
        let url = Self::get_url(media);
        // send request
        let body = reqwest::get(&url)
            .await
            .map_err(|e| anyhow::anyhow!("impossibile ottenere le immagini da {}: {}", url, e))?
            .text()
            .await
            .map_err(|e| {
                anyhow::anyhow!(
                    "impossibile leggere il corpo della pagina da {}: {}",
                    url,
                    e
                )
            })?;
        // parse document
        let document = Html::parse_document(&body);
        // search for entry content selector
        let entry_content_selector = Selector::parse(r#"div[class="entry-content"]"#).unwrap();
        let mut containers = document.select(&entry_content_selector);
        let container = containers.next();
        if container.is_none() {
            anyhow::bail!("la pagina {} è vuota", url)
        }
        let img_selector = Selector::parse("img").unwrap();
        let images = container.unwrap().select(&img_selector);
        let mut urls: Vec<Url> = Vec::new();
        for image in images {
            if let Some(Ok(url)) = image.value().attr("src").map(Url::from_str) {
                // check domain
                if url
                    .domain()
                    .map(|x| x == "ilmondodigrazia.com")
                    .unwrap_or(false)
                {
                    urls.push(url)
                }
            }
        }
        if urls.is_empty() {
            anyhow::bail!("impossibile trovare delle immagini in {}", url);
        }
        Ok(urls)
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[tokio::test]
    async fn should_get_birthday_images() {
        assert!(!IlMondoDiGrazia::default()
            .scrape(Greeting::Compleanno)
            .await
            .unwrap()
            .is_empty());
    }

    #[tokio::test]
    async fn should_get_goodmorning_images() {
        assert!(!IlMondoDiGrazia::default()
            .scrape(Greeting::BuonGiorno)
            .await
            .unwrap()
            .is_empty());
    }

    #[tokio::test]
    async fn should_get_weekday_images() {
        assert!(!IlMondoDiGrazia::default()
            .scrape(Greeting::BuonGiornoWeekday)
            .await
            .unwrap()
            .is_empty());
    }

    #[tokio::test]
    async fn should_get_christmas_images() {
        assert!(!IlMondoDiGrazia::default()
            .scrape(Greeting::Natale)
            .await
            .unwrap()
            .is_empty());
    }

    #[tokio::test]
    async fn should_get_afternoon_images() {
        assert!(!IlMondoDiGrazia::default()
            .scrape(Greeting::BuonPomeriggio)
            .await
            .unwrap()
            .is_empty());
    }

    #[tokio::test]
    async fn should_get_night_images() {
        assert!(!IlMondoDiGrazia::default()
            .scrape(Greeting::BuonaNotte)
            .await
            .unwrap()
            .is_empty());
    }
}
