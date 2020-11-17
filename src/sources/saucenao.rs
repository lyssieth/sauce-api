use crate::{Sauce, SauceItem, SauceResult};
use async_trait::async_trait;
use reqwest::{header, Client};
use serde_derive::Deserialize;
use std::collections::HashMap;

const BASE_URL: &str = "https://saucenao.com/search.php?db=999&output_type=2&testmode=1&numres=16&url={url}&api_key={api_key}";

pub struct SauceNao;

#[async_trait]
impl Sauce for SauceNao {
    async fn check_sauce(url: &str) -> Result<SauceResult, String> {
        let api_key = get_api_key()?;

        let mut vars = HashMap::new();
        vars.insert("url".to_string(), urlencoding::encode(url));
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

fn get_api_key() -> Result<String, String> {
    std::env::var("SAUCENAO_API_KEY").map_err(|e| {
        format!(
            "SAUCENAO_API_KEY environment variable could not be found: {}",
            e
        )
    })
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
