use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Catch-all error type")]
    Any(Box<dyn std::error::Error + Send>),
    #[error("Request error")]
    HttpRequestError(Box<dyn std::error::Error + Send>),
    #[error("HTTP request rate limited")]
    RateLimited(i64), // i64 -> time till limit expires
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Self::HttpRequestError(Box::new(value))
    }
}
