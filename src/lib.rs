use async_trait::async_trait;

pub mod prelude;
pub mod sources;

#[async_trait]
pub trait Sauce {
    async fn check_sauce(url: &str) -> Result<SauceResult, String>;
    async fn check_sauces(urls: Vec<String>) -> Result<Vec<SauceResult>, String> {
        let mut out = Vec::new();
        for x in urls {
            let res = Self::check_sauce(&x).await?;
            out.push(res);
        }

        Ok(out)
    }
}

#[derive(Debug, Clone, PartialOrd, PartialEq, Default)]
pub struct SauceResult {
    pub original_url: String,
    pub items: Vec<SauceItem>,
}

#[derive(Debug, Clone, PartialOrd, PartialEq, Default)]
pub struct SauceItem {
    pub link: String,
    pub similarity: f64,
}
