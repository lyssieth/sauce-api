/// The various errors that this API can produce.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// The provided link does not lead to an image file.
    /// Or the header does not specify that the Content-Type is an image.
    #[error("The provided link does not lead to an image file.")]
    LinkIsNotImage,

    /// Unable to format.
    /// See [strfmt::FmtError]
    #[error("Unable to format string: {0}")]
    UnableToFormat(#[from] strfmt::FmtError),

    /// Unable to convert to string.
    /// See [reqwest::header::ToStrError]
    #[error("Unable to convert to string: {0}")]
    UnableToConvertToString(#[from] reqwest::header::ToStrError),

    /// Unable to convert to float.
    /// See [std::num::ParseFloatError]
    #[error("Unable to convert to float: {0}")]
    UnableToConvertToFloat(#[from] std::num::ParseFloatError),

    /// Failed to send request.
    /// See [reqwest::Error]
    #[error("Failed to send request: {0}")]
    FailedRequest(#[from] reqwest::Error),

    /// Failed to parse into json.
    /// See [serde_json::Error]
    #[error("Failed to parse into json: {0}")]
    FailedToParseIntoJson(#[from] serde_json::Error),

    /// Unable to retrieve sauce.
    /// A more generic error.
    #[error("Unable to retrieve sauce: {0}")]
    UnableToRetrieve(&'static str),

    /// A very generic error, one which couldn't be generalized.
    #[error("An error occurred: {0}")]
    GenericStr(&'static str),

    /// A very generic error, one which couldn't be generalized.
    #[error("An error occurred: {0}")]
    GenericString(String),
}
