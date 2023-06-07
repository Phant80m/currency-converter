use clap::Parser;
use owo_colors::OwoColorize;
mod convert;
use cconvert::{NAME, VERSION};
use convert::{convert, deserialize};
#[derive(Debug, Parser)]
#[command(disable_help_flag = true)]
struct Args {
    #[arg(long)]
    help: bool,
    #[arg(short, long, default_value = "~")]
    input: String,
    #[arg(short, long, default_value = "~")]
    output: String,
    #[arg(short, long, default_value = "0")]
    amount: i32,
}

fn main() {
    let arguments = Args::parse();
    let cconvert = convert(arguments.input.clone());
    if arguments.help {
        help()
    }
    required(&arguments);

    let output = match deserialize(
        cconvert.unwrap_or_default(),
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

fn required(arguments: &Args) {
    if arguments.input == "~" {
        println!(
            "{}",
            "Supply an Argument for input: \n -i | --input <currency>"
                .red()
                .bold()
        );
        std::process::exit(0)
    }
    if arguments.output == "~" {
        println!("Supply an Argument for output: \n -o | --output <currency>");
        std::process::exit(0)
    }

    if arguments.amount == 0 {
        println!("Supply an Argument for amount (value cannot be 0): \n -a | --amount <NUMBER>");
        std::process::exit(0)
    }
}

fn help() {
    let help = format!(
        "│{}                                 │\n│{}                   │\n│{}                  │\n│{}                    │",
        "   Tool Help:".bold().blue(),
        "    -i | --input <CURRENCY>".bold().green(),
        "    -o | --output <CURRENCY>".bold().green(),
        "    -a | --amount <NUMBER>".bold().green(),
    );
    println!(
        "{}",
        "┌──────────────────────────────────────────────┐".blue()
    );
    println!(
        "{}{} {}{}",
        "│           Welcome to ".blue(),
        cconvert::NAME.to_string().purple(),
        cconvert::VERSION.bright_red(),
        "          │".blue(),
    );
    println!(
        "{}",
        "│     A multi-purpose command-line interface   │".blue()
    );
    println!(
        "{}",
        "│          for conversion of currency.         │".blue()
    );

    println!("{help}");

    println!(
        "{}",
        "└──────────────────────────────────────────────┘".blue()
    );
    std::process::exit(0);
}
