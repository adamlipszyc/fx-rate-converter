
use std::collections::HashMap;

use crate::rate_provider::{FxError, RateProvider};

pub struct StaticRateProvider {
    rates: HashMap<(String, String), f64>,
}

impl StaticRateProvider {
    pub fn new() -> Self {
        let mut rates: HashMap<(String, String), f64> = HashMap::new();

        // Hardcode a few rates
        rates.insert(("EUR".to_string(), "USD".to_string()), 1.08);
        rates.insert(("USD".to_string(), "JPY".to_string()), 157.5);
        rates.insert(("GBP".to_string(), "USD".to_string()), 1.27);
        rates.insert(("EUR".to_string(), "GBP".to_string()), 0.86);

        Self { rates }
    }
}


impl RateProvider for StaticRateProvider {
    fn get_rate(&self, base: &str, quote: &str) -> Result<f64, FxError> {
        if let Some(rate) = self.rates.get(&(base.to_string(), quote.to_string())) {
            return Ok(*rate);
        }

        if let Some(opp) = self.rates.get(&(quote.to_string(), base.to_string())) {
            return Ok(*opp);
        }

        Err(FxError::PairNotFound(format!("{base} -> {quote}")))
    }
}