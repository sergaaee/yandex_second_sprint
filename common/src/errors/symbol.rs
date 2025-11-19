use std::time::SystemTimeError;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error, Serialize, Deserialize)]
pub enum SymbolError {
    #[error("unsupported symbol")]
    UnsupportedSymbol,
}

#[derive(Debug, Error)]
pub enum GenerationError {
    #[error("Symbol error occured")]
    SymbolError(#[from] SymbolError),
    #[error("System time error occured")]
    SystemTimeError(#[from] SystemTimeError),
}
