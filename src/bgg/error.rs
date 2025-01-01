use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("{0}")]
    XmlError(String),
    #[error("{0}")]
    HttpError(#[from] reqwest::Error),
    #[error("{0}")]
    XmlApiError(String),
}

pub type Result<T> = std::result::Result<T, Error>;
