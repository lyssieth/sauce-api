#![deny(missing_docs, missing_crate_level_docs, missing_debug_implementations, missing_doc_code_examples, unused)]

//! sauce-api is an API for finding the source image for low-quality or cropped
//! images. Currently it only works with anime-styled images, but I hope to make
//! it capable of doing other kinds of images as well.

pub use async_trait::async_trait;
pub use crate::error::SauceError;

/// Everything important in one nice neat module you can import.
pub mod prelude;

/// The individual sources for the API.
pub mod sources;

/// Contains the various errors that can be produced.
pub mod error;

/// A generic trait that can be used to standardize the sauce system across different sources.
#[async_trait]
pub trait Sauce {
    /// Builds the URL for the given location, allowing one to provide it in case an error happens
    async fn build_url(&self, url: &str) -> Result<String, SauceError>;
    /// Runs the sauce engine against a given URL, providing either results or a 'String' as an error.
    async fn check_sauce(&self, url: String) -> Result<SauceResult, SauceError>;
    /// Just runs check_sauce several times, combining it all into one Vec<SauceResult>
    async fn check_sauces(&self, urls: Vec<String>) -> Result<Vec<SauceResult>, SauceError> {
        let mut out = Vec::new();
        for x in urls {
            let res = self.check_sauce(x).await?;
            out.push(res);
        }

        Ok(out)
    }
}

/// The container for all results provided by sauce-api
#[derive(Debug, Clone, PartialOrd, PartialEq, Default)]
pub struct SauceResult {
    /// The original URL provided to sauce-api
    pub original_url: String,
    /// A vector of results.
    pub items: Vec<SauceItem>,
}

/// An individual item from the results gotten
#[derive(Debug, Clone, PartialOrd, PartialEq, Default)]
pub struct SauceItem {
    /// Link to the item. Note: this is not a direct link to the image, but to a site such as pixiv or danbooru.
    pub link: String,
    /// A similarity, usually as `92.4` or whatever the case may be.
    pub similarity: f64,
}
