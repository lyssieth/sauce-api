use async_trait::async_trait;

use crate::error::Error;

#[cfg(feature = "saucenao")]
/// The source for `saucenao.com`. Requires an API key to function.
pub mod saucenao;

#[cfg(feature = "iqdb")]
/// The source for `iqdb.org`.
pub mod iqdb;

/// The generic trait implemented by all sources under this module.
#[async_trait]
pub trait Source {
    /// The type of the source. Should be `Self`
    type S;
    /// The argument for [`Source::create`]
    type Argument;

    /// Searches for the source of a given URL.
    async fn check(&self, url: &str) -> Result<Output, Error>;

    /// Allows for self-modifying the state of the Source, with an additional 'State' parameter that
    /// can be passed in.
    async fn create(argument: Self::Argument) -> Result<Self::S, Error>;
}

/// The output of a Source.
#[derive(Debug, Clone)]
pub struct Output {
    /// The original URL provided to the Source.
    pub original_url: String,
    /// The results of the search.
    pub items: Vec<Item>,
}

/// An individual item from the results gotten.
#[derive(Debug, Clone)]
pub struct Item {
    /// Link to the item. Note: this is not always a direct link to the image, but to a site such as pixiv or danbooru.
    pub link: String,
    /// A similarity, usually as `92.4` or whatever the case may be.
    ///
    /// # Notes
    /// A negative value means that a similarity could not be parsed.
    pub similarity: f32,
}
