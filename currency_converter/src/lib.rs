use std::env;


pub struct Config {
    pub currency_from: String,
    pub amount: String,
    pub currency_to: String,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();
        let currency_from = match args.next() {
            Some(arg) => arg, 
            None => return Err("Didn't get first currency"),
        };
        let amount = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get an amount."),
        };
        let currency_to = match args.next() {
            Some(arg) => arg, 
            None => return Err("Didn't get second currency."),
        };
        Ok(Config {currency_from, amount, currency_to})
    }
}
