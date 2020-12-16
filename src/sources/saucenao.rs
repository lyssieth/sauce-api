use crate::{Sauce, SauceItem, SauceResult};
use async_trait::async_trait;
use reqwest::{header, Client};
use serde_derive::Deserialize;
use std::collections::HashMap;

const BASE_URL: &str = "https://saucenao.com/search.php?db=999&output_type=2&testmode=1&numres=16&url={url}&api_key={api_key}";

pub struct SauceNao {
    api_key: Option<String>,
}

#[async_trait]
impl Sauce for SauceNao {
    async fn check_sauce(&self, url: String) -> Result<SauceResult, String> {
        let api_key = self.get_api_key();

        if api_key.is_none() {
            return Err("API_KEY is None".to_string());
        }
        let api_key = api_key.clone().unwrap();

        let mut vars = HashMap::new();
        vars.insert("url".to_string(), urlencoding::encode(&url));
        vars.insert("api_key".to_string(), api_key);

        let fmt =
            strfmt::strfmt(BASE_URL, &vars).map_err(|e| format!("Unable to format url: {}", e))?;

        let cli = Client::new();
        let resp = cli
            .get(&fmt)
            .header(header::ACCEPT_ENCODING, "utf-8")
            .send()
            .await
            .map_err(|e| format!("Unable to send request to `{}`: {}", fmt, e))?;

        let res = resp
            .json::<ApiResult>()
            .await
            .map_err(|e| format!("Unable to make ApiResult: {}", e))?;

        let mut result = SauceResult {
            original_url: url.to_string(),
            ..Default::default()
        };

        for x in res.results {
            if let Some(links) = x.data.ext_urls {
                let item = SauceItem {
                    similarity: x
                        .header
                        .similarity
                        .parse::<f64>()
                        .map_err(|e| format!("Unable to parse a float: {}", e))?,
                    link: links[0].clone(),
                };
                result.items.push(item);
            }
        }

        Ok(result)
    }
}

impl SauceNao {
    pub fn new() -> Self {
        SauceNao { api_key: None }
    }

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
