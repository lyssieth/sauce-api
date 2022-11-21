#![warn(clippy::nursery, clippy::pedantic)]
#![deny(
    missing_docs,
    rustdoc::missing_crate_level_docs,
    missing_debug_implementations,
    unused
)]

//! sauce-api is an API for finding the source image for low-quality or cropped
//! images. Currently it only works with anime-styled images, but I hope to make
//! it capable of doing other kinds of images as well.

/// Contains the error type
pub mod error;

/// Contains the various sources
pub mod source;
