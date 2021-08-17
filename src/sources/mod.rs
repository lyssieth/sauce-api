/// Contains the [`IQDB`] source and related data.
#[cfg(feature = "iqdb")]
pub mod iqdb;
/// Contains the [`SauceNao`] source and related data.
#[cfg(feature = "saucenao")]
pub mod saucenao;

#[cfg(feature = "iqdb")]
pub use iqdb::IQDB;
#[cfg(feature = "saucenao")]
pub use saucenao::SauceNao;
