use crate::{Sauce, SauceItem, SauceResult};
use async_trait::async_trait;
use reqwest::Client;
use select::document::Document;
use select::predicate::*;

const BASE_ADDRESS: &str = "https://iqdb.org/";

/// Gets sauces from iqdb.org
pub struct IQDB;

#[async_trait]
impl Sauce for IQDB {
    async fn check_sauce(url: &str) -> Result<SauceResult, String> {
        let cli = Client::new();
        let resp = cli
            .get(BASE_ADDRESS)
            .query(&[("url", url)])
            .send()
            .await
            .map_err(|e| format!("Failed to send request: {}", e))?;

        let resp = resp
            .text()
            .await
            .map_err(|e| format!("Failed to convert to text: {}", e))?;

        let html = Document::from(resp.as_str());

        let mut status = html.find(Attr("id", "urlstat"));

        if let Some(status) = status.next() {
            if !status.text().trim().starts_with("OK, ") {
                return Err(format!("Unable to retrieve sauce: {}", status.text()));
            } else {
                eprintln!("success in getting image: {}", status.text());
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
                            let td = node.first_child().unwrap();
                            let link = td.first_child().unwrap();
                            let href = link.attr("href").unwrap().to_string();
                            item.link = if href.starts_with("//") {
                                "https:".to_string() + &href
                            } else {
                                href
                            };
                        }
                        4 => {
                            let td = node.first_child().unwrap();
                            let text = td.text();
                            let similarity = text.split('%').collect::<Vec<&str>>()[0];
                            item.similarity = similarity.parse::<f64>().unwrap();
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
            let real_pages = pages.find(Class("pages")).next().unwrap();

            for node in real_pages.children() {
                let mut item = SauceItem::default();

                for (idx, node) in node.find(Name("tr")).enumerate() {
                    match idx {
                        0 => {
                            let td = node.first_child().unwrap();
                            let link = td.first_child().unwrap();
                            let href = link.attr("href").unwrap().to_string();
                            item.link = if href.starts_with("//") {
                                "https:".to_string() + &href
                            } else {
                                href
                            };
                        }
                        1..=2 => continue,
                        3 => {
                            let td = node.first_child().unwrap();
                            let text = td.text();
                            let similarity = text.split('%').collect::<Vec<&str>>()[0];
                            item.similarity = similarity.parse::<f64>().unwrap();
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
