use crate::{Error, Sauce, SauceItem, SauceResult};
use async_trait::async_trait;
use reqwest::{header, Client};
use select::document::Document;
use select::node::Node;
use select::predicate::*;

const BASE_ADDRESS: &str = "https://iqdb.org/";

/// Gets sauces from iqdb.org
#[derive(Debug)]
pub struct IQDB;

#[async_trait]
impl Sauce for IQDB {
    async fn build_url(&self, url: &str) -> Result<String, Error> {
        Ok(format!("{}?url={}", BASE_ADDRESS, url))
    }

    async fn check_sauce(&self, url: &str) -> Result<SauceResult, Error> {
        let cli = Client::new();

        let head = cli.head(url).send().await?;

        let content_type = head.headers().get(header::CONTENT_TYPE);
        if let Some(content_type) = content_type {
            let content_type = content_type.to_str()?;
            if !content_type.contains("image") {
                return Err(Error::LinkIsNotImage);
            }
        }

        let resp = cli.get(&self.build_url(url).await?).send().await?;

        let resp = resp.text().await?;

        let html = Document::from(resp.as_str());

        let mut status = html.select(Attr("id", "urlstat"));

        if let Some(status) = status.next() {
            if !status.text().trim().starts_with("OK, ") {
                return Err(Error::GenericString(format!(
                    "Unable to retrieve sauce: {}",
                    status.text()
                )));
            }
        }

        let mut res = SauceResult {
            original_url: url.to_string(),
            ..SauceResult::default()
        };

        let mut pages = html.select(Attr("id", "pages"));

        let pages = pages.next();

        Self::harvest_stage_one(pages, &mut res)?;

        let pages = html.select(Attr("id", "more1")).next();

        Self::harvest_stage_two(pages, &mut res)?;

        Ok(res)
    }
}

impl IQDB {
    fn harvest_stage_one(pages: Option<Node>, res: &mut SauceResult) -> Result<(), Error> {
        let mut first = false;

        if let Some(pages) = pages {
            for node in pages.children() {
                if !first {
                    first = true;
                    continue;
                }
                let mut item = SauceItem::default();

                for (idx, node) in node.select(Name("tr")).enumerate() {
                    match idx {
                        0 | 2..=3 => continue,
                        1 => {
                            let td_or_th = node.first_child();
                            if td_or_th.is_none() {
                                continue;
                            }
                            let td_or_th = td_or_th.unwrap();

                            if td_or_th.is(Name("th")) {
                                break;
                            }

                            let link = td_or_th.first_child();
                            if link.is_none() {
                                break;
                            }
                            let href = link.unwrap().attr("href");
                            if href.is_none() {
                                break;
                            }
                            let href = href.unwrap();
                            item.link = if href.starts_with("//") {
                                "https:".to_string() + href
                            } else {
                                href.to_string()
                            };
                        }
                        4 => {
                            let td = node.first_child();
                            if td.is_none() {
                                continue;
                            }
                            let td = td.unwrap();
                            let text = td.text();
                            let similarity = text.split('%').collect::<Vec<&str>>()[0];
                            item.similarity = match similarity.parse::<f32>() {
                                Ok(similarity) => Ok(similarity),
                                Err(e) => Err(Error::UnableToConvertToFloat(e)),
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
        };

        Ok(())
    }

    fn harvest_stage_two(pages: Option<Node>, res: &mut SauceResult) -> Result<(), Error> {
        if let Some(pages) = pages {
            let real_pages = match pages.select(Class("pages")).next() {
                Some(real_pages) => Ok(real_pages),
                None => Err(Error::UnableToRetrieve("failed to parse page")),
            }?;

            for node in real_pages.children() {
                let mut item = SauceItem::default();

                for (idx, node) in node.select(Name("tr")).enumerate() {
                    match idx {
                        0 => {
                            let td = match node.first_child() {
                                Some(node) => Ok(node),
                                None => Err(Error::UnableToRetrieve("failed to parse page")),
                            }?;
                            let link = match td.first_child() {
                                Some(node) => Ok(node),
                                None => Err(Error::UnableToRetrieve("failed to parse page")),
                            }?;
                            let href = match link.attr("href") {
                                Some(href) => Ok(href.to_string()),
                                None => Err(Error::UnableToRetrieve("failed to parse page")),
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
                                None => Err(Error::UnableToRetrieve("failed to parse page")),
                            }?;
                            let text = td.text();
                            let similarity = text.split('%').collect::<Vec<&str>>()[0];
                            item.similarity = match similarity.parse::<f32>() {
                                Ok(similarity) => Ok(similarity),
                                Err(e) => Err(Error::UnableToConvertToFloat(e)),
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
        };

        Ok(())
    }
}
