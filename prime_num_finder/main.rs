// Write a programme which will print all the pairs of prime numbers whose sum equals the number entered by the user. 
// Eg 10 = 5 + 5, 7 + 3; 12 = 11 + 1, 5 + 7

use primes::{Sieve, PrimeSet};
use std::io;

fn main() {
    let mut pset = Sieve::new();
    let mut primes = Vec::new();
    
    println!("Input a number please.");
    let mut input_str = String::new();
    io::stdin().read_line(&mut input_str).expect("Failed to read input.");
    let input: u64 = input_str.trim().parse().expect("Failed to parse string");

    for (_ix,n) in pset.iter().enumerate().take(250) {
        if n < input - 1 {
            primes.push(n);
        }    
    }
    println!("Prime numbers are {:?}", primes);
    let mut pairs: Vec<(&u64, &u64)> = Vec::new();
    for x in primes.iter() {
        for y in primes[0..primes.len()/2].iter() {
            if input == x + y {
                pairs.push((y, x));
            }    
        }
    }
    println!("Pairs are: {:?}", pairs);
}
