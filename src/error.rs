use std::fmt;

/// Errors for sauce-api
#[derive(Debug)]
pub enum Error {
    /// The provided link does not lead to an image file, or the Content-Type is unspecified.
    LinkIsNotImage,

    /// A generic error, aka something in the pipeline went wrong
    Generic(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LinkIsNotImage => write!(f, "The provided link does not lead to an image file, or the Content-Type is unspecified."),
            Self::Generic(s) => write!(f, "{}", s),
        }
    }
}

macro_rules! impl_from {
    ($from:ty) => {
        impl From<$from> for Error {
            fn from(e: $from) -> Self {
                Self::Generic(e.to_string())
            }
        }
    };
}

impl_from!(reqwest::Error);
impl_from!(reqwest::header::ToStrError);
impl_from!(serde_json::Error);
impl_from!(std::num::ParseFloatError);
impl_from!(Box<dyn std::error::Error + std::marker::Send + std::marker::Sync>);
