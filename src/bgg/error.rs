use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Invalid user")]
    InvalidUserError,
    #[error("{0}")]
    XmlError(String),
    #[error("{0}")]
    HttpError(String),
    #[error("{0}")]
    XmlApiError(String),
    #[error("{0}")]
    IoError(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
