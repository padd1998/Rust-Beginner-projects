use currency_converter::Config;
use std::env;
use std::process;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    println!("Converting {} to {}", config.currency_from, config.currency_to);

    if config.currency_to == String::from("pounds") && config.currency_from == String::from("dollars") {
        let dollars = config.amount.parse().expect("Failed to parse amount in dollars.");
        let pounds = dollars_to_pounds(dollars);
        println!("You have {} pounds.", pounds);
    } else if config.currency_to == String::from("dollars") && config.currency_from == String::from("pounds") {
        let pounds = config.amount.parse().expect("Failed to parse amount in pounds.");
        let dollars = pounds_to_dollars(pounds);
        println!("You have {} dollars.", dollars);
    } else if config.currency_to == String::from("euros") && config.currency_from == String::from("pounds") {
        let pounds = config.amount.parse().expect("Failed to parse amount in pounds.");
        let euros = pounds_to_euros(pounds);
        println!("You have {}", euros);
    } else if config.currency_to == String::from("pounds") && config.currency_from == String::from("euros") {
        let euros = config.amount.parse().expect("Failed to parse amount in euros.");
        let pounds = euros_to_pounds(euros);
        println!("You have {}", pounds);
    }
}

fn pounds_to_dollars(pounds: f64) -> f64 {
    let dollars = pounds * 1.32;
    dollars
}

fn dollars_to_pounds(dollars: f64) -> f64 {
    let pounds = dollars / 1.32;
    pounds
}
fn euros_to_pounds(euros: f64) -> f64 {
    let pounds = euros / 1.19;
    pounds
}

fn pounds_to_euros(pounds: f64) -> f64 {
    let euros = pounds * 1.19;
    euros
}
