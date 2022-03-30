use std::io;

fn main() {
    println!("Please input the number. ");
    let mut guess = String::new();
    io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line.");

    let num: u64 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => 6,
    };
    let answer = factorial(num);
    println!("Answer is {}", answer);
}

fn factorial(num: u64) -> u64 {
    if num <= 1 {
        return 1
    }
    num * factorial(num - 1)
}
