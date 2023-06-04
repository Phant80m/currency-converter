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
    #[arg(long, short)]
    input: String,
    #[arg(short, long)]
    output: String,
    #[arg(short, long)]
    amount: i32,
}

fn main() {
    let arguments = Args::parse();
    let _cconvert = convert(arguments.input.clone());

    let output = deserialize(_cconvert, arguments.output.to_uppercase(), arguments.amount);

    println!(
        "{} {} = {} {}",
        arguments.amount.purple().bold(),
        arguments.input.to_uppercase().red().bold(),
        output.purple().bold(),
        arguments.output.to_uppercase().green().bold()
    );
}

fn convert(currency: String) -> String {
    let body: String =
        ureq::get(format!("https://api.exchangerate-api.com/v4/latest/{}", currency).as_str())
            .call()
            .expect("Failed to call fn")
            .into_string()
            .expect("failed to stringify");
    body
}

// for example Currency * amount
fn deserialize(input: String, currency: String, amount: i32) -> f64 {
    let jcurrency: Jcurrency = serde_json::from_str(input.as_str()).expect("");

    let cout = jcurrency.rates[currency.as_str()]
        .as_f64()
        .expect("failed to floatify it");

    cout * amount as f64
}
