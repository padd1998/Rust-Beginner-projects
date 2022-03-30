use std::env;
use std::process;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err|{
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    let val:f64 = config.value.parse().expect("Unable to parse value string.");
    if config.dimension == String::from("radius") {
        radius(val);
    } else if config.dimension == String::from("diameter") {
        diameter(val);
    } else if config.dimension == String::from("area") {
        area(val);
    }
}

fn area(a: f64) {
    let d = (a / 3.14).sqrt();
    println!("Diameter is {}", d);
    let r = d / 2.0;
    println!("Radius is {}", r);
}

fn diameter(d: f64) {
    let radius = d / 2.0;
    println!("Radius is {}", radius);
    let area = 3.14 * radius * radius;
    println!("Area is {}", area);
}

fn radius(r: f64) {
    let area = 3.14 * r * r;
    println!("Area is {}", area);
    let diameter = 2.0 * r;
    println!("Diameter is {}", diameter);
}

struct Config {
    dimension: String,
    value: String,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();
        let dimension = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a string."),
        };
        let value = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a value.")
        };
        return Ok(Config{dimension, value})
    }
}
