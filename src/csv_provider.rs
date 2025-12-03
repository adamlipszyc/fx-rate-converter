use std::collections::HashMap;
use std::fs::File;
use std::io::{ BufRead, BufReader };

use crate::rate_provider::{FxError, RateProvider};

pub struct CsvRateProvider {
    rates: HashMap<(String, String), f64>,
}

impl CsvRateProvider {
    pub fn from_csv(path: &str) -> Result<Self, FxError> {
        let file = File::open(path)
            .map_err(|e| FxError::CsvError(format!("Failed to open {path}: {e}")))?;

        let reader = BufReader::new(file);
        let mut rates = HashMap::new();

        for (idx, line) in reader.lines().enumerate() {
            let line = line.map_err(|e| FxError::CsvError(format!("IO error: {e}")))?;

            if idx == 0 {
                continue;
            }

            let parts: Vec<_> = line.split(',').collect();
            if parts.len() != 3 {
                return Err(FxError::CsvError(format!("Bad row: {line}")));
            }

            let base = parts[0].trim().to_string();
            let quote = parts[1].trim().to_string();

            let rate: f64 = parts[2]
                .trim()
                .parse()
                .map_err(|_| FxError::ParseError(format!("Bad rate: {}", parts[2])))?;

            rates.insert((base, quote), rate);
        }
        Ok(Self { rates })
    }
}

impl RateProvider for CsvRateProvider {
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
