use std::net::AddrParseError;
use thiserror::Error;
use common::errors::symbol::SymbolError;

#[derive(Debug, Error)]
pub enum ServerError {
    #[error("Symbol error occurred")]
    SymbolError(#[from] SymbolError),
    #[error("Invalid UDP address")]
    AddrParseError(#[from] AddrParseError),
}
