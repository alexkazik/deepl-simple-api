use reqwest::StatusCode;
use std::fmt::{Display, Formatter};

/// Errors returned by the API.
#[derive(Debug)]
pub enum Error {
    /// When an empty slice is passed to `translate` (the `DeepL` API is not called).
    NoTexts,
    /// When calling `translate`, `text` must be passed as parameter, not in the options.
    OptionTextSet,
    /// When calling `translate`, `text` must be passed as parameter, not in the options.
    OptionContextSet,
    /// When `translate` is called without a target language (the `DeepL` API is not called).
    NoTargetLanguage,
    /// Status code 413.
    RequestSizeExceedsTheLimit,
    /// Status code 429.
    TooManyRequests,
    /// Status code 456.
    QuotaExceeded,
    /// Not mapped status code.
    UnknownStatus {
        /// `StatusCode` of the response
        status: StatusCode,
        /// Textual representation of the response body (or None if there was a problem loading it)
        text: Option<String>,
    },
    /// An error in the underlying reqwest engine, e.g. no route, decoding failed.
    ReqwestError(reqwest::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::NoTexts => f.write_str("no texts"),
            Error::NoTargetLanguage => f.write_str("no target language"),
            Error::OptionTextSet => f.write_str("option text is set"),
            Error::OptionContextSet => f.write_str("option context is set"),
            Error::RequestSizeExceedsTheLimit => f.write_str("request size exceeds the limit"),
            Error::TooManyRequests => f.write_str("too many requests"),
            Error::QuotaExceeded => f.write_str("quota exceeded"),
            Error::UnknownStatus { status, text } => {
                write!(
                    f,
                    "unknown status {status}: {}",
                    text.as_ref().map_or("<Error loading body>", |x| x.as_str())
                )
            }
            Error::ReqwestError(error) => write!(f, "reqwest error: {error}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        if let Error::ReqwestError(error) = self {
            error.source()
        } else {
            None
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Error::ReqwestError(value)
    }
}
