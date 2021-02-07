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
        let params = [
            ("url", url),
            ("rpt", "imageview"),
            ("format", "json"),
            (
                "request",
                "{\"blocks\":[{\"block\":\"b-page_type_search-by-image__link\"}]}",
            ),
        ];

        let cli = Client::new();

        let resp = cli.get(BASE_ADDRESS).query(&params).send().await?;

        let text = resp.text().await?;

        return if text.contains("captcha") {
            let json: YandexCaptchaUrl = serde_json::from_str(&text)?;

            Err(SauceError::HitByCaptcha(json))
        } else {
            let json: YandexBuildUrl = serde_json::from_str(&text)?;

            Ok(format!("{}?{}", BASE_ADDRESS, json.blocks[0].params.url))
        };
    }

    async fn check_sauce(&self, original_url: &str) -> Result<SauceResult, SauceError> {
        let url = self.build_url(&original_url).await?;

        let cli = Client::new();

        let resp = cli.get(&url).send().await?;

        let html = Document::from(resp.text().await?.as_str());

        let view_other_sizes = html.find(Class("Tags-Wrapper")).into_selection();

        if !view_other_sizes.is_empty() {
            let mut res = Vec::new();
            for x in view_other_sizes.children().iter() {
                if let Some(link) = x.attr("href") {
                    if link.contains("i.ytimg.com") {
                        continue; // We skip i.ytimg.com because it is dynamic and generally bad.
                    }
                    res.push(SauceItem {
                        link: link.to_string(),
                        similarity: -1.0,
                    });
                }
            }

            Ok(SauceResult {
                original_url: original_url.to_string(),
                items: res,
            })
        } else {
            Err(SauceError::UnableToRetrieve(
                "Could not find any similar images",
            ))
        }
    }
}

/// Contains the full JSON return from the captcha
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct YandexCaptchaUrl {
    /// Type of some kind (unknown)
    r#type: String,
    /// Details
    /// See [YandexCaptcha]
    captcha: YandexCaptcha,
}

/// Contains the details of the captcha
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct YandexCaptcha {
    /// URL to the captcha's image
    #[serde(alias = "img-url")]
    img_url: String,
    /// Key for the captcha
    key: String,
    /// Captcha's status
    status: String,
    /// URL to link to the captcha.
    #[serde(alias = "captcha-page")]
    captcha_page: String,
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
