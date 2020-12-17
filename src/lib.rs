use async_trait::async_trait;

/// Everything important in one nice neat module you can import.
pub mod prelude;

/// The individual sources for the API.
pub mod sources;

/*
@todo Add better error types for a more complete API
@body Currently the errors are simple String types, so this could be improved massively with concise error typing.
*/

/// A generic trait that can be used to standardize the sauce system across different sources.
#[async_trait]
pub trait Sauce {
    /// Builds the URL for the given location, allowing one to provide it in case an error happens
    async fn build_url(&self, url: &str) -> Result<String, String>;
    /// Runs the sauce engine against a given URL, providing either results or a 'String' as an error.
    async fn check_sauce(&self, url: String) -> Result<SauceResult, String>;
    /// Just runs check_sauce several times, combining it all into one Vec<SauceResult>
    async fn check_sauces(&self, urls: Vec<String>) -> Result<Vec<SauceResult>, String> {
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
