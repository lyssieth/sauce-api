use std::time::Duration;

use async_trait::async_trait;
use reqwest::header;
use visdom::{
    types::{BoxDynElement, Elements},
    Vis,
};

use crate::{error::Error, make_client};

use super::{Item, Output, Source};

/// The [`IQDB`] source.
///
/// Works with `iqdb.org`
#[derive(Debug)]
pub struct Iqdb;

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

        let html = Vis::load(text)?;

        let pages = html.find("#pages").children("div");

        let best_match = if pages.length() > 2 {
            Self::harvest_best_match(&pages.eq(1))
        } else {
            None
        };

        let mut items = Vec::new();

        if let Some(best_match) = best_match {
            items.push(best_match);
        }

        for page in pages.into_iter().skip(2) {
            let page = Self::harvest_page(&page);

            items.extend(page);
        }

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
    fn harvest_page(page: &BoxDynElement) -> Option<Item> {
        let dom = Vis::dom(page);

        let link = dom.find(".image a").first();

        let url = link.attr("href")?;

        let score = dom.find("tr");

        if score.length() != 5 {
            return Some(Item {
                link: url.to_string(),
                similarity: -1.0,
            });
        }

        let score = score.eq(4);
        let td = score.find("td");

        let score = td.html();
        let score = score.split_once('%')?.0.parse::<f32>().ok()? / 100.0;

        Some(Item {
            link: url.to_string(),
            similarity: score,
        })
    }

    fn harvest_best_match(pages: &Elements) -> Option<Item> {
        let link = pages.find(".image a").first();

        let url = link.attr("href")?;

        let score = pages.find("tr");

        if score.length() != 5 {
            return Some(Item {
                link: url.to_string(),
                similarity: -1.0,
            });
        }

        let score = score.eq(4);
        let td = score.find("td");

        let score = td.html();
        let score = score.split_once('%')?.0.parse::<f32>().ok()? / 100.0;

        Some(Item {
            link: url.to_string(),
            similarity: score,
        })
    }
}
