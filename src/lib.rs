#![warn(clippy::nursery, clippy::pedantic)]
#![deny(
    missing_docs,
    rustdoc::missing_crate_level_docs,
    missing_debug_implementations,
    unused
)]
#![doc = include_str!("../README.md")]

/// Contains the error type
pub mod error;

/// Contains the various sources
pub mod source;
