use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, EnumIter, EnumString};

pub mod quote;
pub mod symbol;

// This is strange separetion on models and implementations, maybe it is common in C# with its DDD but in rust we usually follow simple module hierarchy approach but this one still works well
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockQuote {
    pub ticker: String,
    pub price: f64,
    pub volume: u32,
    pub timestamp: u64,
}

#[derive(
    Debug, Serialize, Deserialize, AsRefStr, EnumIter, PartialEq, Eq, Hash, Clone, Copy, EnumString,
)]
#[serde(rename_all = "UPPERCASE")]
#[strum(serialize_all = "UPPERCASE")]
pub enum Symbol {
    AAPL,
    MSFT,
    GOOGL,
    AMZN,
    META,
    TSLA,
    V,
    PG,
    HD,
    DIS,
    #[serde(other)]
    Unknown,
}
