use clap::Parser;
use owo_colors::OwoColorize;
mod convert;
use convert::{convert, deserialize};
#[derive(Debug, Parser)]
#[command(disable_help_flag = false)]
struct Args {
    #[arg(short, long)]
    input: String,
    #[arg(short, long)]
    output: String,
    #[arg(short, long)]
    amount: i32,
}

#[tokio::main]
async fn main() {
    let arguments = Args::parse();
    let cconvert = convert(arguments.input.to_owned());

    let output = match deserialize(
        cconvert.await.unwrap_or_default(),
        arguments.output.to_uppercase(),
        arguments.amount,
    )
    .await
    {
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
