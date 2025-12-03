use crate::{csv_provider::CsvRateProvider, rate_provider::{FxError, RateProvider}, static_provider::StaticRateProvider};

mod rate_provider;
mod csv_provider;
mod static_provider;

fn main() -> Result<(), FxError> {
    let provider = CsvRateProvider::from_csv("./data/rates.csv")?;



    let base = "EUR";
    let quote = "USD";
    let amount_str = "100.0";

    let mut history = ConversionHistory {
        queries: Vec::new()
    };


    let amount: f64= match amount_str.parse() {
        Ok(am) => am,
        Err(_) => {
            eprintln!("Invalid amount entered: {amount_str}");
            return Err(FxError::ParseError(format!("Invalid amount entered: {amount_str}")))
        }
    };

    let converted = convert_fx(&provider, amount, base, quote);

    match converted {
        Ok(result) => {
            let rate = provider.get_rate(base, quote).unwrap();
            println!("Converting {amount} {base} -> {quote}");
            println!("Rate: {rate}");
            println!("Result: {result} {quote}");
            Ok(())
        }
        Err(FxError::PairNotFound(pair)) => {
            eprintln!("Unknown currency pair: {pair}");
            Err(FxError::PairNotFound(pair))
        }
        Err(FxError::InvalidAmount(msg)) => {
            eprintln!("Invalid amount: {msg}");
            Err(FxError::InvalidAmount(msg))
        }
        Err(FxError::CsvError(msg)) => {
            eprintln!("CsvError: {msg}");
            Err(FxError::CsvError(msg))
        }
        Err(FxError::ParseError(msg)) => {
            eprintln!("ParseError: {msg}");
             Err(FxError::ParseError(msg))
        }
    }

}

struct ConversionHistory {
    queries: Vec<(String, String, f64, f64)>,
}


fn convert_fx<P>(
    provider: &P,
    amount: f64,
    base: &str,
    quote: &str,
) -> Result<f64, FxError> 
where 
    P: RateProvider,
{
    let rate = provider.get_rate(base, quote)?;
    Ok(amount * rate)
}
