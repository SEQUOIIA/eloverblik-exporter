use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Catch-all error type")]
    Any(Box<dyn std::error::Error + Send>),
    #[error("Request error")]
    RequestError(Box<dyn std::error::Error + Send>),
    #[error("serde_json error")]
    SerdeJsonError(Box<dyn std::error::Error + Send>),
    #[error("eloverblik_client error")]
    ElOverblikClientError(Box<dyn std::error::Error + Send>),
    #[error("config error")]
    ConfigError(Box<dyn std::error::Error + Send>),
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Self::RequestError(Box::new(value))
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::SerdeJsonError(Box::new(value))
    }
}

impl From<config::ConfigError> for Error {
    fn from(value: config::ConfigError) -> Self {
        Self::ConfigError(Box::new(value))
    }
}

impl From<eloverblik_client::error::Error> for Error {
    fn from(value: eloverblik_client::error::Error) -> Self {
        Self::ElOverblikClientError(Box::new(value))
    }
}