use common::errors::symbol::GenerationError;
use common::models::{StockQuote, Symbol};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct QuoteGenerator {
    pub last_prices: HashMap<Symbol, f64>,
}

impl QuoteGenerator {
    pub fn new() -> Self {
        Self {
            last_prices: HashMap::new(),
        }
    }

    pub fn generate_quote(&mut self, symbol: Symbol) -> Result<StockQuote, GenerationError> {
        symbol.validate()?;

        let last_price = self
            .last_prices
            .entry(symbol)
            .or_insert(100.0 + rand::random::<f64>() * 50.0);

        // Симулируем колебание цены
        let delta = (rand::random::<f64>() - 0.5) * 2.0; // [-1.0 .. 1.0]
        *last_price = (*last_price + delta).max(0.01);

        let volume = match symbol {
            Symbol::AAPL | Symbol::MSFT | Symbol::TSLA => {
                1000 + (rand::random::<f64>() * 5000.0) as u32
            }
            _ => 100 + (rand::random::<f64>() * 1000.0) as u32,
        };

        let ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)?
            .as_millis() as u64;

        Ok(StockQuote {
            ticker: symbol.as_ref().to_string(),
            price: *last_price,
            volume,
            timestamp: ts,
        })
    }
}
