use crate::{Sauce, SauceItem, SauceResult};
use async_trait::async_trait;
use reqwest::Client;
use select::document::Document;
use select::predicate::*;
use serde_derive::{Deserialize, Serialize};

const BASE_ADDRESS: &str = "https://yandex.com/images/search";

/// Gets sauces from yandex.com
pub struct Yandex;

#[async_trait]
impl Sauce for Yandex {
    async fn build_url(&self, url: &str) -> Result<String, String> {
        let blocks = r#"{"blocks":[{"block":"b-page_type_search-by-image__link"}]}"#;
        let get_url = format!(
            r#"{}?url={}&rpt=imageview&format=json&request={}"#,
            BASE_ADDRESS, url, blocks
        );

        let cli = Client::new();

        let resp = cli
            .get(&get_url)
            .send()
            .await
            .map_err(|e| format!("Failed to send request: {}", e))?;

        let json = resp
            .json::<YandexBuildUrl>()
            .await
            .map_err(|e| format!("Failed to parse request: {}", e))?;

        Ok(format!("{}?{}", BASE_ADDRESS, json.blocks[0].params.url))
    }

    async fn check_sauce(&self, url: String) -> Result<SauceResult, String> {
        let url = self.build_url(&url).await?;

        let cli = Client::new();

        let resp = cli
            .get(&url)
            .send()
            .await
            .map_err(|e| format!("Failed to send request: {}", e))?;

        let html = Document::from(
            resp.text()
                .await
                .map_err(|e| format!("Unable to convert to text: {}", e))?
                .as_str(),
        );

        let _similar = html.find(And(Class("CbirItem"), Class("CbirOtherSizes")));

        todo!("Currently unimplemented")
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
