use std::io;

fn main() {
    println!("Enter a sequence of characters. ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line.");
    let mut vowels: u32 = 0;
    let mut consonants: u32 = 0;
    for i in input.chars() {
        let thing = i.to_string();
        if i.is_alphabetic() { 
            if is_vowel(&thing) {
                vowels += 1;
            } else {
            consonants += 1;
            }
        }
    }
    println!("You entered {} consonants.", consonants);
    println!("You entered {} vowels.", vowels);
}

fn is_vowel(c: &str) -> bool {
    if "aeiou".contains(&c.to_lowercase()) {
        return true
    } else {
        false
    } 
}
