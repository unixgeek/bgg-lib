use thiserror::Error;

/// Possible errors when using the API.
#[derive(Debug, Error)]
pub enum Error {
    #[error("Invalid user")]
    /// The user does not exist in the XML API.
    InvalidUserError,
    #[error("{0}")]
    /// There was an error processing XML.
    XmlError(String),
    #[error("{0}")]
    /// There was an error making the HTTP request.
    HttpError(#[from] ureq::Error),
    #[error("{0}")]
    /// The XML API responded with an error.
    XmlApiError(String),
}

pub type Result<T> = std::result::Result<T, Error>;
