#[derive(Debug)]
pub enum FxError {
    PairNotFound(String),
    InvalidAmount(String),
    CsvError(String),
    ParseError(String),
}

pub trait RateProvider {
    fn get_rate(&self, base: &str, quote: &str) -> Result<f64, FxError>;
}
