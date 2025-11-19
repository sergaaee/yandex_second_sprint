use thiserror::Error;

#[derive(Debug, Error)]
pub enum QuoteError {
    #[error("Couldn't parse JSON: {0}")]
    ParseError(#[from] serde_json::Error),
}
