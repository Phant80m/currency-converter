use std::io::stdin;

fn main() {
    // maim 
    let mut choice = String::new();
    println!("Convert [aud] to [usd] [gbp] [cad], choices:\n usd\n gbp\n cad \n");
    std::io::stdin().read_line(&mut choice).expect("failed to read line");

    if choice.trim() == "usd" {
        aud2usd();
    } else if choice.trim() == "gbp"{
        aud2gbp();
    } else if choice.trim() == "cad"{
        aud2cad();
    }

    fn aud2usd() {
        let usd: f64 = 0.681359;
        let mut aud = String::new();
        std::io::stdin().read_line(&mut aud).expect("cannot read line");
        let number: f64 = aud.trim().parse().expect("Cannot Parse line. Is it a float?");
        println!("the value in USD is: {:.2}", number * usd);
    }
    fn aud2gbp() {
        let gbp: f64 =  0.5637;
        let mut aud = String::new();
        std::io::stdin().read_line(&mut aud).expect("error reading line.");
        let number: f64 = aud.trim().parse().expect("cannot parse line. Is it a float?");
        println!("the value in GBP (british pound) is: {:.2}", number * gbp);
    }
    fn aud2cad() {
        let cad: f64 = 0.92249222;
        let mut aud = String::new();
        std::io::stdin().read_line(&mut aud).expect("cannot read line");
        let input: f64 = aud.trim().parse().expect("Failed to read character. Is it a float?");
        println!("the value in CAD (canadian dollar) is: {}", input * cad);
    }


}
