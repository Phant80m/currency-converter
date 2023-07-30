use owo_colors::OwoColorize;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::task;
#[derive(Serialize, Deserialize)]
struct Jcurrency {
    rates: Value,
}

pub async fn convert(currency: String) -> Result<String, String> {
    let result = task::spawn(async move {
        let body: String =
            ureq::get(format!("https://api.exchangerate-api.com/v4/latest/{}", currency).as_str())
                .call()
                .map_err(|err| format!("{} {}", "Failed to call function".red().bold(), err))?
                .into_string()
                .map_err(|err| format!("{} {}", "Failed to index Currency".red().bold(), err))?;
        Ok(body)
    })
    .await;

    result.unwrap()
}
// for example Currency * amount
pub async fn deserialize(input: String, currency: String, amount: i32) -> Result<f64, String> {
    let error: String = format!(
        "{}",
        "one or more Currencies are unknown:\nDoes it exist?"
            .bold()
            .red()
    );

    let jcurrency: Jcurrency = match serde_json::from_str(input.as_str()) {
        Ok(currency_out) => currency_out,
        Err(_) => return Err(error),
    };
    let result = task::spawn_blocking({
        let error = error.clone();
        move || {
            jcurrency.rates[currency.as_str()]
                .as_f64()
                .ok_or(error)
                .map(|rate| rate * amount as f64)
        }
    })
    .await;

    match result {
        Ok(result) => result.into(),
        Err(_) => Err(error.clone()),
    }
}
