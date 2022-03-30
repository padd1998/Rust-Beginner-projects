use std::io;

fn main() {
    println!("Input a word.");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line.");
    let mut itera:Vec<char> = input.chars().collect();
    let length = itera.len() - 2;
    itera = itera[0..length].to_vec();

    let thing = itera.clone();
    let mut rev_iter = Vec::new();
    for i in itera {
        rev_iter.insert(0 , i);
    }
    if thing == rev_iter {
        println!("{} is a palindrome", input.trim());
    } else {
        println!("{} is NOT a palindrome.", input.trim());
    }
}
