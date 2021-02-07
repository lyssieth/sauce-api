pub use crate::prelude::*;
pub use tokio::test;

#[cfg(feature = "iqdb")]
mod iqdb_tests;

#[cfg(feature = "saucenao")]
mod saucenao_tests;

#[cfg(feature = "yandex")]
mod yandex_tests;
