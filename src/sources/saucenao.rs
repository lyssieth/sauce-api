use crate::{Sauce, SauceError, SauceItem, SauceResult};
use async_trait::async_trait;
use reqwest::{header, Client};
use serde::Deserialize;
use std::collections::HashMap;

const BASE_URL: &str = "https://saucenao.com/search.php?db=999&output_type=2&testmode=1&numres=16&url={url}&api_key={api_key}";

/// The SauceNao source.
/// Requires an API key to function.
#[derive(Debug)]
pub struct SauceNao {
    api_key: Option<String>,
}

#[async_trait]
impl Sauce for SauceNao {
    async fn build_url(&self, url: &str) -> Result<String, SauceError> {
        let api_key = self.get_api_key();

        if api_key.is_none() {
            return Err(SauceError::GenericStr("API_KEY is None"));
        }
        let api_key = api_key.clone().unwrap();

        let mut vars = HashMap::new();
        vars.insert("url".to_string(), urlencoding::encode(&url));
        vars.insert("api_key".to_string(), api_key);

        let fmt = strfmt::strfmt(BASE_URL, &vars)?;

        return Ok(fmt);
    }

    async fn check_sauce(&self, original_url: String) -> Result<SauceResult, SauceError> {
        let url = self.build_url(&original_url).await?;

        let cli = Client::new();
        let head = cli.head(&original_url).send().await?;

        let content_type = head.headers().get(header::CONTENT_TYPE);
        if let Some(content_type) = content_type {
            let content_type = content_type.to_str()?;
            if !content_type.contains("image") {
                return Err(SauceError::LinkIsNotImage);
            }
        }

        let resp = cli
            .get(&url)
            .header(header::ACCEPT_ENCODING, "utf-8")
            .send()
            .await?;

        let res = resp.json::<ApiResult>().await?;

        let mut result = SauceResult {
            original_url: original_url.to_string(),
            ..Default::default()
        };

        for x in res.results {
            if let Some(links) = x.data.ext_urls {
                let item = SauceItem {
                    similarity: x.header.similarity.parse::<f64>()?,
                    link: links[0].clone(),
                };
                result.items.push(item);
            }
        }

        Ok(result)
    }
}

impl SauceNao {
    /// Creates a new SauceNao source, with a [None] api key.
    pub fn new() -> Self {
        SauceNao { api_key: None }
    }

    /// Sets the api key to a given [String].
    pub fn set_api_key(&mut self, api_key: String) {
        self.api_key = Some(api_key)
    }

    fn get_api_key(&self) -> &Option<String> {
        &self.api_key
    }
}

impl Default for SauceNao {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Deserialize, PartialOrd, PartialEq, Clone, Default)]
struct ApiResult {
    pub header: ApiHeader,
    pub results: Vec<ApiResultItem>,
}

#[derive(Debug, Deserialize, PartialOrd, PartialEq, Clone, Default)]
struct ApiHeader {
    pub status: i64,
    pub results_requested: i64,
}

#[derive(Debug, Deserialize, PartialOrd, PartialEq, Clone, Default)]
struct ApiResultItem {
    pub header: ApiResultItemHeader,
    pub data: ApiResultItemData,
}

#[derive(Debug, Deserialize, PartialOrd, PartialEq, Clone, Default)]
struct ApiResultItemHeader {
    pub similarity: String,
}

#[derive(Debug, Deserialize, PartialOrd, PartialEq, Clone, Default)]
struct ApiResultItemData {
    pub ext_urls: Option<Vec<String>>,
}
