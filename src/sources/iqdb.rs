use crate::{Sauce, SauceError, SauceItem, SauceResult};
use async_trait::async_trait;
use reqwest::{header, Client};
use select::document::Document;
use select::predicate::*;

const BASE_ADDRESS: &str = "https://iqdb.org/";

/// Gets sauces from iqdb.org
#[derive(Debug)]
pub struct IQDB;

#[async_trait]
impl Sauce for IQDB {
    async fn build_url(&self, url: &str) -> Result<String, SauceError> {
        Ok(format!("{}?url={}", BASE_ADDRESS, url))
    }

    async fn check_sauce(&self, original_url: String) -> Result<SauceResult, SauceError> {
        let url = &original_url;
        let cli = Client::new();

        let head = cli.head(&original_url).send().await?;

        let content_type = head.headers().get(header::CONTENT_TYPE);
        if let Some(content_type) = content_type {
            let content_type = content_type.to_str()?;
            if !content_type.contains("image") {
                return Err(SauceError::LinkIsNotImage);
            }
        }

        let resp = cli.get(&self.build_url(url).await?).send().await?;

        let resp = resp.text().await?;

        let html = Document::from(resp.as_str());

        let mut status = html.find(Attr("id", "urlstat"));

        if let Some(status) = status.next() {
            if !status.text().trim().starts_with("OK, ") {
                return Err(SauceError::GenericString(format!(
                    "Unable to retrieve sauce: {}",
                    status.text()
                )));
            }
        }

        let mut res = SauceResult {
            original_url: url.to_string(),
            ..Default::default()
        };

        let mut pages = html.find(Attr("id", "pages"));

        let mut first = false;

        let pages = pages.next();

        if let Some(pages) = pages {
            for node in pages.children() {
                if !first {
                    first = true;
                    continue;
                }
                let mut item = SauceItem::default();

                for (idx, node) in node.find(Name("tr")).enumerate() {
                    match idx {
                        0 | 2..=3 => continue,
                        1 => {
                            let td = match node.first_child() {
                                Some(node) => Ok(node),
                                None => Err(SauceError::UnableToRetrieve("failed to parse page")),
                            }?;
                            let link = match td.first_child() {
                                Some(node) => Ok(node),
                                None => Err(SauceError::UnableToRetrieve("failed to parse page")),
                            }?;
                            let href = match link.attr("href") {
                                Some(href) => Ok(href.to_string()),
                                None => Err(SauceError::UnableToRetrieve("failed to parse page")),
                            }?;
                            item.link = if href.starts_with("//") {
                                "https:".to_string() + &href
                            } else {
                                href
                            };
                        }
                        4 => {
                            let td = match node.first_child() {
                                Some(node) => Ok(node),
                                None => Err(SauceError::UnableToRetrieve("failed to parse page")),
                            }?;
                            let text = td.text();
                            let similarity = text.split('%').collect::<Vec<&str>>()[0];
                            item.similarity = match similarity.parse::<f64>() {
                                Ok(similarity) => Ok(similarity),
                                Err(e) => Err(SauceError::UnableToConvertToFloat(e)),
                            }?;
                        }
                        _ => break,
                    }
                }

                if item.link.is_empty() {
                    continue;
                }

                res.items.push(item);
            }
        }

        let pages = html.find(Attr("id", "more1")).next();

        if let Some(pages) = pages {
            let real_pages = match pages.find(Class("pages")).next() {
                Some(real_pages) => Ok(real_pages),
                None => Err(SauceError::UnableToRetrieve("failed to parse page")),
            }?;

            for node in real_pages.children() {
                let mut item = SauceItem::default();

                for (idx, node) in node.find(Name("tr")).enumerate() {
                    match idx {
                        0 => {
                            let td = match node.first_child() {
                                Some(node) => Ok(node),
                                None => Err(SauceError::UnableToRetrieve("failed to parse page")),
                            }?;
                            let link = match td.first_child() {
                                Some(node) => Ok(node),
                                None => Err(SauceError::UnableToRetrieve("failed to parse page")),
                            }?;
                            let href = match link.attr("href") {
                                Some(href) => Ok(href.to_string()),
                                None => Err(SauceError::UnableToRetrieve("failed to parse page")),
                            }?;
                            item.link = if href.starts_with("//") {
                                "https:".to_string() + &href
                            } else {
                                href
                            };
                        }
                        1..=2 => continue,
                        3 => {
                            let td = match node.first_child() {
                                Some(node) => Ok(node),
                                None => Err(SauceError::UnableToRetrieve("failed to parse page")),
                            }?;
                            let text = td.text();
                            let similarity = text.split('%').collect::<Vec<&str>>()[0];
                            item.similarity = match similarity.parse::<f64>() {
                                Ok(similarity) => Ok(similarity),
                                Err(e) => Err(SauceError::UnableToConvertToFloat(e)),
                            }?;
                        }
                        _ => break,
                    }
                }

                if item.link.is_empty() {
                    continue;
                }

                res.items.push(item);
            }
        }

        Ok(res)
    }
}
