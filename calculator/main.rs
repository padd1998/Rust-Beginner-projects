use std::io; 

fn main() {
    println!("Input your first number please.");
    let mut first_str = String::new();
    io::stdin().read_line(&mut first_str).expect("Failed to read line.");
    let first = match first_str.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };
    println!("Input the operation that you want. + - * /");
    let mut oper = String::new();
    io::stdin().read_line(&mut oper).expect("Failed to read operator.");
    // let oper = match operat {
    //     Ok(thing) => thing,
    //     Err(_) => "+",
    // };


    println!("Input the second number.");
    let mut second_str = String::new();
    io::stdin().read_line(&mut second_str).expect("Failed to read second number.");
    let second = match second_str.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    if oper.trim() == "*" {
        multiply(first, second);
    } else if oper.trim() == "+" {
        add(first, second);
    } else if oper.trim() == "-" {
        subtract(first, second);
    } else if oper.trim() == "/" {
        division(first, second);
    }
}

fn multiply(first: u64, second: u64) -> () {
    let answer = first * second;
    println!("{} times {} = {}", first, second, answer);
}
fn division(first: u64, second: u64) -> () {
    let answer = first / second;
    println!("{} divided by {} = {}", first, second, answer);
}
fn subtract(first: u64, second: u64) -> () {
    let answer = first - second;
    println!("{} - {} = {}", first, second, answer);
}
fn add(first: u64, second: u64) -> () {
    let answer = first + second;
    println!("{} + {} = {}", first, second, answer);
}
