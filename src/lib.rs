#![warn(clippy::nursery, clippy::pedantic, clippy::perf)]
#![deny(
    missing_docs,
    rustdoc::missing_crate_level_docs,
    missing_debug_implementations,
    unused,
    clippy::unwrap_used
)]
#![doc = include_str!("../README.md")]

use reqwest::Client;

/// Contains the error type
pub mod error;

/// Contains the various sources
pub mod source;

pub(crate) fn make_client() -> Client {
    Client::builder()
        .user_agent(format!("lyssieth/sauce-api v{}", env!("CARGO_PKG_VERSION")))
        .build()
        .expect("failed to build client")
}
