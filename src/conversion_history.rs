use csv::WriterBuilder;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter};

use crate::rate_provider::FxError;

pub struct ConversionHistory {
    path: String,
    queries: Vec<(String, String, f64, f64)>,
}

impl ConversionHistory {
    pub fn from_csv(path: &str) -> Result<Self, FxError> {
        let file = File::open(path)
            .map_err(|e| FxError::CsvError(format!("Failed to open {path}: {e}")))?;

        let reader = BufReader::new(file);

        let mut history = ConversionHistory {
            path: path.to_string(),
            queries: Vec::new(),
        };

        for (idx, line) in reader.lines().enumerate() {
            let line = line.map_err(|e| FxError::CsvError(format!("IO error: {e}")))?;

            if idx == 0 {
                continue;
            }

            let parts: Vec<_> = line.split(',').collect();
            if parts.len() != 4 {
                return Err(FxError::CsvError(format!("Bad row: {line}")));
            }

            let base = parts[0].trim().to_string();
            let quote = parts[1].trim().to_string();

            let rate: f64 = parts[2]
                .trim()
                .parse()
                .map_err(|_| FxError::ParseError(format!("Bad rate: {}", parts[2])))?;

            let amount: f64 = parts[3]
                .trim()
                .parse()
                .map_err(|_| FxError::ParseError(format!("Bad rate: {}", parts[3])))?;

            history.queries.push((base, quote, rate, amount))
        }

        Ok(history)
    }

    pub fn add_to_csv(
        &self,
        base: &str,
        quote: &str,
        amount: &f64,
        result: &f64,
    ) -> Result<(), FxError> {
        let file = OpenOptions::new()
            .create(true) // create file if it doesn't exist
            .append(true) // append to the end
            .open(&self.path)
            .map_err(|e| {
                FxError::CsvError(format!("Failed to open {} for writing: {e}", self.path))
            })?;

        let buffer = BufWriter::new(file);

        let mut writer = WriterBuilder::new().from_writer(buffer);

        writer
            .write_record(&[base, quote, &amount.to_string(), &result.to_string()])
            .map_err(|e| FxError::CsvError(format!("Unable to write to csv: {e}")))?;

        Ok(())
    }
}
