use crate::errors::symbol::SymbolError;
use crate::models::Symbol;
use strum::IntoEnumIterator;

impl Symbol {
    pub fn validate(&self) -> Result<(), SymbolError> {
        match self {
            Symbol::Unknown => Err(SymbolError::UnsupportedSymbol),
            _ => Ok(()),
        }
    }

    pub fn get_all_symbols() -> Vec<Self> {
        Symbol::iter().collect()
    }
}
