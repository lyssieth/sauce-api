use std::time::Duration;

use async_trait::async_trait;
use reqwest::header;
use scraper::ElementRef;
use tracing::debug;

use crate::{error::Error, make_client};

use super::{Item, Output, Source};

/// The [`IQDB`] source.
///
/// Works with `iqdb.org`
#[derive(Debug)]
pub struct Iqdb;

/// A macro that creates a &Selector from a string literal.
macro_rules! sel {
    ($sel:literal) => {
        &scraper::Selector::parse($sel).expect("invalid selector")
    };
}

#[async_trait]
impl Source for Iqdb {
    type State = ();

    async fn check(&self, url: &str) -> Result<Output, Error> {
        let client = make_client();

        // Check whether we're dealing with an image
        let head = client.head(url).send().await?;

        let content_type = head.headers().get(header::CONTENT_TYPE);

        if let Some(content_type) = content_type {
            let content_type = content_type.to_str()?;

            if !content_type.contains("image") {
                return Err(Error::LinkIsNotImage);
            }
        } else {
            return Err(Error::LinkIsNotImage);
        }

        // Build the request

        let req = client
            .get("https://iqdb.org/")
            .query(&[("url", url)])
            .timeout(Duration::from_secs(10));

        let resp = req.send().await?;

        let text = resp.text().await?;

        let html = scraper::Html::parse_document(&text);

        let mut items: Vec<Item> = html
            .select(sel!("#pages > div"))
            .skip(1)
            .filter_map(Self::harvest_page)
            .collect();

        for item in &mut items {
            if item.link.starts_with("//") {
                item.link = format!("https:{}", item.link);
            }
        }

        Ok(Output {
            original_url: url.to_string(),
            items,
        })
    }

    async fn create(_: Self::State) -> Result<Self, Error> {
        Ok(Self)
    }
}

impl Iqdb {
    fn harvest_page(page: ElementRef) -> Option<Item> {
        debug!("selecting .image a");
        let link = page.select(sel!(".image a")).next()?;

        debug!("grabbing href");
        let url = link.value().attr("href")?;

        debug!("grabbing score");
        let score = page.select(sel!("tr:last-child > td")).next()?;

        debug!("parsing score");
        let score = score.text().collect::<String>();
        let score = score.split_once('%')?.0.parse::<f32>().ok()? / 100.0;

        Some(Item {
            link: url.to_string(),
            similarity: score,
        })
    }
}
