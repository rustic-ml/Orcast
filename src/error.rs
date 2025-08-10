use thiserror::Error;

#[derive(Debug, Error)]
pub enum OrcastError {
    #[error("configuration error: {0}")]
    Config(String),
    #[error("http client error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("not implemented: {0}")]
    NotImplemented(&'static str),
}

pub type Result<T> = std::result::Result<T, OrcastError>;


