use crate::errors::quote::QuoteError;
use crate::models::StockQuote;

impl StockQuote {
    pub fn to_json(&self) -> Result<serde_json::Value, QuoteError> {
        let result = serde_json::to_value(self)?;
        Ok(result)
    }

    pub fn from_json(data: serde_json::Value) -> Option<Self> {
        serde_json::from_value(data).ok()
    }
}
