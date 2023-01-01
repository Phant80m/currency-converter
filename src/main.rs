use std::io::stdin;

fn main() {
    // maim 
    let mut choice = String::new();
    println!("Convert [aud] to [usd] [gbp] [cad], choices:\n usd\n gbp\n cad \n");
    std::io::stdin().read_line(&mut choice).expect("failed to read line");

    let currency = choice.trim();
    match currency {
        "usd" => convert("usd", 0.681359),
        "gbp" => convert("gbp", 0.5637),
        "cad" => convert("cad", 0.92249222),
        _ => println!("Invalid choice"),
    }
}

fn convert(currency: &str, rate: f64) {
    let mut aud = String::new();
    std::io::stdin().read_line(&mut aud).expect("cannot read line");
    let number: f64 = aud.trim().parse().expect("Cannot Parse line. Is it a float?");
    println!("the value in {} is: {:.2}", currency, number * rate);
}
