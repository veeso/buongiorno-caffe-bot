//! # Il mondo di grazia
//!
//! This provider provides images from <https://ilmondodigrazia.com>

use std::str::FromStr;

use super::{Media, Scrape, Url};

use async_trait::async_trait;
use chrono::{Datelike, Local, Weekday};
use scraper::{Html, Selector};

const AUGURI_URL: &str = "https://ilmondodigrazia.com/compleanno";
const BUONGIORNO_URL: &str = "https://ilmondodigrazia.com/buongiorno";
const BUONGIORNO_WEEKDAY_BASE_URL: &str = "https://ilmondodigrazia.com/buongiorno/buongiorno-";
const BUON_NATALE_URL: &str =
    "https://ilmondodigrazia.com/frasi-di-buon-natale-immagini-da-condividere";
const BUON_POMERIGGIO_URL: &str = "https://ilmondodigrazia.com/buon-pomeriggio";
const BUONA_NOTTE_URL: &str = "https://ilmondodigrazia.com/buonanotte";

#[derive(Default)]
pub struct IlMondoDiGrazia;

impl IlMondoDiGrazia {
    fn weekday() -> &'static str {
        match Local::today().weekday() {
            Weekday::Mon => "lunedi",
            Weekday::Tue => "martedi",
            Weekday::Wed => "mercoledi",
            Weekday::Thu => "venerdi",
            Weekday::Fri => "venerdi",
            Weekday::Sat => "sabato",
            Weekday::Sun => "domenica",
        }
    }

    fn get_url(media: Media) -> String {
        match media {
            Media::Auguri => AUGURI_URL.to_string(),
            Media::BuonGiorno => BUONGIORNO_URL.to_string(),
            Media::BuonGiornoWeekday => {
                format!("{}{}", BUONGIORNO_WEEKDAY_BASE_URL, Self::weekday())
            }
            Media::BuonNatale => BUON_NATALE_URL.to_string(),
            Media::BuonPomeriggio => BUON_POMERIGGIO_URL.to_string(),
            Media::BuonaNotte => BUONA_NOTTE_URL.to_string(),
        }
    }
}

#[async_trait]
impl Scrape for IlMondoDiGrazia {
    async fn scrape(&self, media: Media) -> anyhow::Result<Vec<Url>> {
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
            .scrape(Media::Auguri)
            .await
            .unwrap()
            .is_empty());
    }

    #[tokio::test]
    async fn should_get_goodmorning_images() {
        assert!(!IlMondoDiGrazia::default()
            .scrape(Media::BuonGiorno)
            .await
            .unwrap()
            .is_empty());
    }

    #[tokio::test]
    async fn should_get_weekday_images() {
        assert!(!IlMondoDiGrazia::default()
            .scrape(Media::BuonGiornoWeekday)
            .await
            .unwrap()
            .is_empty());
    }

    #[tokio::test]
    async fn should_get_christmas_images() {
        assert!(!IlMondoDiGrazia::default()
            .scrape(Media::BuonNatale)
            .await
            .unwrap()
            .is_empty());
    }

    #[tokio::test]
    async fn should_get_afternoon_images() {
        assert!(!IlMondoDiGrazia::default()
            .scrape(Media::BuonPomeriggio)
            .await
            .unwrap()
            .is_empty());
    }

    #[tokio::test]
    async fn should_get_night_images() {
        assert!(!IlMondoDiGrazia::default()
            .scrape(Media::BuonaNotte)
            .await
            .unwrap()
            .is_empty());
    }
}
