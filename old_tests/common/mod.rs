pub use sauce_api::prelude::*;
pub use tokio::test;

// This is the URL we test for.
// It is art by 'shirleydraws' on pixiv
// https://www.pixiv.net/member_illust.php?mode=medium&illust_id=85559849
pub const TEST_URL: &str = "https://i.imgur.com/vRsNUMS.jpg";

// This is used as a stand-in for SauceItem
// So we can compare more easily.
pub struct TestItem {
    pub link: &'static str,
    pub similarity: f32,
}
