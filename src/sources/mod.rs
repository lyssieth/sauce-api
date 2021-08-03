/// Contains the IQDB source and related data.
#[cfg(feature = "iqdb")]
pub mod iqdb;
/// Contains the SauceNao source and related data.
#[cfg(feature = "saucenao")]
pub mod saucenao;
/// Contains the Yandex source and related data.
#[cfg(feature = "yandex")]
#[deprecated(
    since = "0.8.0",
    note = "This is jank and always was jank, please stop using it."
)]
pub mod yandex;

#[cfg(feature = "iqdb")]
pub use iqdb::IQDB;
#[cfg(feature = "saucenao")]
pub use saucenao::SauceNao;
#[cfg(feature = "yandex")]
pub use yandex::Yandex;
