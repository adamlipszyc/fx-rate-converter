use std::env;

use crate::{
    csv_provider::CsvRateProvider,
    rate_provider::{FxError, RateProvider},
    static_provider::StaticRateProvider,
    conversion_history::ConversionHistory
};

mod csv_provider;
mod rate_provider;
mod static_provider;
mod conversion_history;

fn main() -> Result<(), FxError> {
    let provider = CsvRateProvider::from_csv("./data/rates.csv")?;

    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        eprintln!("Usage:");
        eprintln!("  {} <base_currency> <quote_currency> <amount>", args[0]);
        eprintln!("Example:");
        eprintln!("  {} EUR USD 100.0", args[0]);
        return Err(FxError::ParseError("Invalid arguments".to_string()));
    }



    let base = &args[1];
    let quote = &args[2];
    let amount_str = &args[3];

    let mut history = ConversionHistory::from_csv("./data/history.csv")?;
    let amount: f64 = match amount_str.parse() {
        Ok(am) => am,
        Err(_) => {
            eprintln!("Invalid amount entered: {amount_str}");
            return Err(FxError::InvalidAmount(format!(
                "Invalid amount entered: {amount_str}"
            )));
        }
    };

    let converted = convert_fx(&provider, amount, base, quote);

    

    match converted {
        Ok(result) => {
            let rate = provider.get_rate(base, quote).unwrap();
            println!("Converting {amount} {base} -> {quote}");
            println!("Rate: {rate}");
            println!("Result: {result} {quote}");
            history.add_to_csv(base, quote, &amount, &result)?;
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


fn convert_fx<P>(provider: &P, amount: f64, base: &str, quote: &str) -> Result<f64, FxError>
where
    P: RateProvider,
{
    let rate = provider.get_rate(base, quote)?;
    Ok(amount * rate)
}
