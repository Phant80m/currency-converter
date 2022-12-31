fn main() {
    // maim 
    let mut choice = String::new();
    println!("Convert [aud] to [usd] or [gbp], choices:\n usd\n gbp\n ");
    std::io::stdin().read_line(&mut choice).expect("failed to read line");

    if choice.trim() == "usd" {
        aud2usd();
    } else if choice.trim() == "gbp"{
        aud2gbp();
    }

    fn aud2usd() {
        // us dollar float: (used to compare to the input user gives:)
        let usd: f64 = 0.681359;
    
        // get user input 
        let mut aud = String::new();
        std::io::stdin().read_line(&mut aud).expect("cannot read line");
    
        // converts the string to a float ( f64):
        let number: f64 = aud.trim().parse().expect("Cannot Parse line. Is it a string?");
    
        // prints the output value in USD:sd
        println!("the value in USD is: {:.2}", number * usd);
    }
    fn aud2gbp() {
        let gbp: f64 =  0.5637;
        let mut aud = String::new();
        std::io::stdin().read_line(&mut aud).expect("error reading line.");
        let number: f64 = aud.trim().parse().expect("cannot parse line. Is it a string?");
        println!("the value in GBP (british pound) is: {:.2}", number * gbp);
    }

}
