use serde::de::DeserializeOwned;
use std::{collections::HashMap, string::ToString};

pub use types::*;

mod types;

/// `FuzzySearch` is a collection of methods to get information from fuzzysearch.net.
pub struct FuzzySearch {
    endpoint: String,
    api_key: String,
    client: reqwest::Client,
}

/// How to match against `FuzzySearch`.
#[derive(Debug, PartialEq, Eq)]
pub enum MatchType {
    /// Start by looking at only exact items, then expand if no results.
    Close,
    /// Only look at exact items.
    Exact,
    /// Force matching expanded set of results.
    Force,
}

pub struct FuzzySearchOpts {
    pub endpoint: Option<String>,
    pub client: Option<reqwest::Client>,
    pub api_key: String,
}

impl FuzzySearch {
    pub const API_ENDPOINT: &'static str = "https://api-next.fuzzysearch.net/v1";

    /// Create a new `FuzzySearch` instance. Requires the API key.
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            client: reqwest::Client::new(),
            endpoint: Self::API_ENDPOINT.to_string(),
        }
    }

    /// Create a new `FuzzySearch` instance with a custom client or endpoint.
    pub fn new_with_opts(opts: FuzzySearchOpts) -> Self {
        Self {
            api_key: opts.api_key,
            client: opts.client.unwrap_or_default(),
            endpoint: opts
                .endpoint
                .unwrap_or_else(|| Self::API_ENDPOINT.to_string()),
        }
    }

    /// Makes a request against the API. It deserializes the JSON response.
    /// Generally not used as there are more specific methods available.
    async fn make_request<T: Default + DeserializeOwned>(
        &self,
        endpoint: &str,
        params: &HashMap<&str, String>,
    ) -> reqwest::Result<T> {
        let url = format!("{}{}", self.endpoint, endpoint);

        let req = self
            .client
            .get(&url)
            .header("x-api-key", self.api_key.as_bytes())
            .query(params);

        let req = Self::trace_headers(req);

        req.send().await?.json().await
    }

    /// Attempt to lookup multiple hashes.
    #[tracing::instrument(err, skip(self))]
    pub async fn lookup_hashes(
        &self,
        hashes: &[i64],
        distance: Option<i64>,
    ) -> reqwest::Result<Vec<File>> {
        let mut params = HashMap::new();
        params.insert(
            "hash",
            hashes
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(","),
        );
        if let Some(distance) = distance {
            params.insert("distance", distance.to_string());
        }

        self.make_request("/hashes", &params).await
    }

    /// Attempt to perform a search using an image URL.
    #[tracing::instrument(err, skip(self))]
    pub async fn lookup_url(&self, url: &str) -> reqwest::Result<Vec<File>> {
        let mut params = HashMap::new();
        params.insert("url", url.to_string());

        self.make_request("/url", &params).await
    }

    /// Attempt to reverse image search.
    ///
    /// Requiring an exact match will be faster, but potentially leave out results.
    #[tracing::instrument(err, skip(self, data))]
    pub async fn image_search(
        &self,
        data: &[u8],
        exact: MatchType,
        distance: Option<i64>,
    ) -> reqwest::Result<Vec<File>> {
        use reqwest::multipart::{Form, Part};

        let url = format!("{}/image", self.endpoint);

        let part = Part::bytes(Vec::from(data));
        let form = Form::new().part("image", part);

        let mut query = match exact {
            MatchType::Exact => vec![("type", "exact".to_string())],
            MatchType::Force => vec![("type", "force".to_string())],
            MatchType::Close => vec![("type", "close".to_string())],
        };
        if let Some(distance) = distance {
            query.push(("distance", distance.to_string()));
        }

        let req = self
            .client
            .post(&url)
            .query(&query)
            .header("x-api-key", self.api_key.as_bytes())
            .multipart(form);

        let req = Self::trace_headers(req);

        req.send().await?.json().await
    }

    /// Attempt to resolve some information from a `FurAffinity` file.
    #[tracing::instrument(err, skip(self))]
    pub async fn lookup_furaffinity_file(
        &self,
        url: &str,
    ) -> reqwest::Result<Vec<FurAffinityFileDetail>> {
        let mut params = HashMap::new();
        params.insert("search", url.to_string());

        self.make_request("/file/furaffinity", &params).await
    }

    const fn trace_headers(req: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        req
    }
}
