use clap::Parser;
use owo_colors::OwoColorize;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
struct Jcurrency {
    rates: Value,
}

#[derive(Debug, Parser)]
struct Args {
    #[arg(short, long)]
    input: String,
    #[arg(short, long)]
    output: String,
    #[arg(short, long)]
    amount: i32,
}

fn main() {
    let arguments = Args::parse();
    let _cconvert = convert(arguments.input.clone());

    let output = match deserialize(
        _cconvert.unwrap_or_default(),
        arguments.output.to_uppercase(),
        arguments.amount,
    ) {
        Ok(value) => value,
        Err(err) => {
            println!("{}", err);
            return;
        }
    };

    println!(
        "{} {} = {} {}",
        arguments.amount.purple().bold(),
        arguments.input.to_uppercase().red().bold(),
        output.bold().purple(),
        arguments.output.to_uppercase().green().bold()
    );
}

fn convert(currency: String) -> Result<String, String> {
    let body: String =
        ureq::get(format!("https://api.exchangerate-api.com/v4/latest/{}", currency).as_str())
            .call()
            .map_err(|err| format!("{} {}", "Failed to call function".red().bold(), err))?
            .into_string()
            .map_err(|err| format!("{} {}", "Failed to index Currency".red().bold(), err))?;
    Ok(body)
}

// for example Currency * amount
fn deserialize(input: String, currency: String, amount: i32) -> Result<f64, String> {
    let jcurrency: Jcurrency = match serde_json::from_str(input.as_str()) {
        Ok(currency_out) => currency_out,
        Err(_) => {
            return Err(format!(
                "{} {} {}",
                "one or more Currencies are unknown:".bold().red(),
                input.bold().yellow(),
                "\nDoes it exist?".bold().red()
            ))
        }
    };

    let cout = jcurrency.rates[currency.as_str()].as_f64().ok_or_else(|| {
        format!(
            "{} {} {}",
            "Unknown Currency:".bold().red(),
            currency.bold().yellow(),
            "\nHave you spelled it right?".bold().red(),
        )
    })?;

    Ok(cout * amount as f64)
}
