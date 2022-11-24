use async_trait::async_trait;
use reqwest::{header, Client};
use serde::{Deserialize, Serialize};

use crate::error::Error;

use super::{Item, Output, Source};

/// The [`SauceNao`] source.
/// Requires an API key to function.
///
/// Works with `saucenao.com`
#[derive(Debug)]
pub struct SauceNao {
    /// The API key to use.
    api_key: String,
}

#[async_trait]
impl Source for SauceNao {
    type Argument = String;

    async fn check(&self, url: &str) -> Result<Output, Error> {
        let client = Client::new();

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

        let req = {
            client
                .get("https://saucenao.com/search.php")
                .query(&Query::default().url(url).api_key(&self.api_key))
                .header(header::ACCEPT_ENCODING, "utf-8")
        };

        // Send the request

        let resp = req.send().await?;

        // Parse the response

        let text = resp.text().await?;
        let json: ApiResponse = serde_json::from_str(&text)?;

        let mut result = Output {
            original_url: url.to_string(),
            items: Vec::new(),
        };

        for item in json.results {
            if let Some(links) = item.data.ext_urls {
                let item = Item {
                    similarity: item.header.similarity.parse::<f32>()?,
                    link: links[0].clone(),
                };

                result.items.push(item);
            }
        }

        Ok(result)
    }

    async fn create(arg: Self::Argument) -> Result<Self, Error> {
        Ok(Self { api_key: arg })
    }
}

#[derive(Debug, Serialize)]
struct Query {
    url: String,
    api_key: String,
    db: u16,
    output_type: u8,
    #[serde(rename = "testmode")]
    test_mode: u8,
    #[serde(rename = "numres")]
    num_res: u8,
}

impl Query {
    pub fn url(mut self, url: &str) -> Self {
        self.url = url.to_string();
        self
    }

    pub fn api_key(mut self, api_key: &str) -> Self {
        self.api_key = api_key.to_string();
        self
    }
}

impl Default for Query {
    fn default() -> Self {
        Self {
            url: String::new(),
            api_key: String::new(),
            db: 999,
            output_type: 2,
            test_mode: 1,
            num_res: 16,
        }
    }
}

#[derive(Debug, Deserialize)]
struct ApiResponse {
    pub results: Vec<ApiItem>,
}

#[derive(Debug, Deserialize)]
struct ApiItem {
    pub header: ApiItemHeader,
    pub data: ApiItemData,
}

#[derive(Debug, Deserialize)]
struct ApiItemHeader {
    pub similarity: String,
}

#[derive(Debug, Deserialize)]
struct ApiItemData {
    pub ext_urls: Option<Vec<String>>,
}
