use std::fmt::Debug;

use async_trait::async_trait;
use fuzzysearch::{FuzzySearch as FuzzySearchInternal, FuzzySearchOpts};
use reqwest::{header, StatusCode};
use serde::{Deserialize, Serialize};
use time::PrimitiveDateTime;
use tracing::{debug, warn};

use crate::{error::Error, make_client};

use super::{Item, Output, Source};

/// The [`FuzzySearch`] source.
///
/// Works with `https://fuzzysearch.net`
pub struct FuzzySearch {
    internal: FuzzySearchInternal,
}

impl Debug for FuzzySearch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FuzzySearch").finish()
    }
}

#[async_trait]
#[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation)]
impl Source for FuzzySearch {
    type State = String;

    async fn check(&self, url: &str) -> Result<Output, Error> {
        let client = make_client();

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
        let resp = self.internal.lookup_url(url).await;

        // Check the status
        if let Err(e) = resp {
            let status = e.status().expect("A status code should be present");

            warn!(?e, "Got error from fuzzysearch");

            match status {
                StatusCode::BAD_REQUEST => {
                    return Err(Error::Generic("URL invalid or too large".to_string()))
                }
                StatusCode::UNAUTHORIZED => {
                    return Err(Error::Generic("API key invalid or missing".to_string()))
                }
                StatusCode::TOO_MANY_REQUESTS => {
                    return Err(Error::Generic("Rate limit exhausted".to_string()))
                }

                _ => return Err(Error::Generic(format!("Unexpected status code: {status}"))),
            };
        }

        let results = resp?; // Handle any other error

        debug!(?results, "Got results");

        // Convert the response to the output format

        let mut output = Output {
            original_url: url.to_string(),
            items: Vec::new(),
        };

        for result in results {
            let distance = result.distance.unwrap_or(0);

            let item = Item {
                link: result.url,
                similarity: 100f32 / ((distance + 1) * 100) as f32,
            };

            output.items.push(item);
        }

        output
            .items
            .sort_unstable_by_key(|i| (i.similarity * 100f32) as i32);

        Ok(output)
    }

    async fn create(state: Self::State) -> Result<Self, Error> {
        Ok(Self {
            internal: FuzzySearchInternal::new_with_opts(FuzzySearchOpts {
                api_key: state,
                client: Some(make_client()),
                endpoint: Some("https://api.fuzzysearch.net".to_string()),
            }),
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct SearchResult {
    site: String,
    site_info: Option<SiteInfo>,
    artists: Vec<String>,
    distance: u32,
    filename: String,
    hash: u32,
    hash_str: String,
    posted_at: PrimitiveDateTime,
    rating: String,
    searched_hash: u32,
    searched_hash_str: String,
    sha256: String,
    site_id: u32,
    site_id_str: String,
    tags: Vec<String>,
    url: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct SiteInfo {
    file_id: u32,
}
