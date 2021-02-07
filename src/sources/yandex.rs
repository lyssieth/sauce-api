use crate::{Sauce, SauceError, SauceItem, SauceResult};
use async_trait::async_trait;
use reqwest::Client;
use select::document::Document;
use select::predicate::*;
use serde::{Deserialize, Serialize};

const BASE_ADDRESS: &str = "https://yandex.com/images/search";

/// Gets sauces from yandex.com
#[derive(Debug)]
pub struct Yandex;

#[async_trait]
impl Sauce for Yandex {
    async fn build_url(&self, url: &str) -> Result<String, SauceError> {
        let blocks = r#"{"blocks":[{"block":"b-page_type_search-by-image__link"}]}"#;
        let get_url = format!(
            r#"{}?url={}&rpt=imageview&format=json&request={}"#,
            BASE_ADDRESS, url, blocks
        );

        let cli = Client::new();

        let resp = cli.get(&get_url).send().await?;

        let json = resp.json::<YandexBuildUrl>().await?;

        Ok(format!("{}?{}", BASE_ADDRESS, json.blocks[0].params.url))
    }

    async fn check_sauce(&self, url: String) -> Result<SauceResult, SauceError> {
        let url = self.build_url(&url).await?;

        let cli = Client::new();

        let resp = cli.get(&url).send().await?;

        let html = Document::from(resp.text().await?.as_str());

        let view_other_sizes = html.find(Class("Tags-Wrapper")).into_selection();

        if !view_other_sizes.is_empty() {
            let mut res = Vec::new();
            for x in view_other_sizes.children().iter() {
                if let Some(link) = x.attr("href") {
                    res.push(SauceItem {
                        link: link.to_string(),
                        similarity: -1.0,
                    });
                }
            }

            Ok(SauceResult {
                original_url: url,
                items: res,
            })
        } else {
            Err(SauceError::UnableToRetrieve(
                "Could not find any similar images",
            ))
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct YandexBuildUrl {
    pub blocks: Vec<YandexBlock>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct YandexBlock {
    pub params: YandexBlockParams,
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct YandexBlockParams {
    pub url: String,
}
