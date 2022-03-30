use rand::Rng;
use std::io;

fn main() {
    // Generate 2 random numbers between 1-14
    let mut pounds = 100;
    println!("Welcome to blackjack.");
    
    while pounds > 0 {
        println!("You have {} pounds.", pounds);
        let card: u32 = rand::thread_rng().gen_range(1..11);
        let card_2: u32 = rand::thread_rng().gen_range(1..12);
        // Add together
        let mut total = card + card_2;
        // Ask user if they want to hit or stop
        println!("How much do you want to bet on this hand?");
        let bet = input_int();
        println!("You have {}. Hit (type hit) or stand (type stand)?", total);
        let mut choice = input();
        
        while choice == "hit".to_string() {
            let new_card: u32 = rand::thread_rng().gen_range(1..12);
            total = total + new_card;
        
            if total > 21 {
                break
            } else {
                println!("You have {}. Hit (type hit) or stand (type stand)?", total);
                choice = input();
            }
        }

        let dealer_1 = rand::thread_rng().gen_range(1..12);
        let dealer_2 = rand::thread_rng().gen_range(1..12);
        let count = dealer_1 + dealer_2;
        let dealer = dealer(count);
        
        pounds = win_or_lose(total, dealer, pounds, bet);
    }
}

fn dealer(mut count: u32) -> u32 {
    while count < 16 {
        count = count + rand::thread_rng().gen_range(1..12);
    }
    return count;
}

fn win_or_lose(total: u32, dealer: u32, mut pounds: u32, bet: u32) -> u32 {
    if total > 21 {
        println!("You have gone BUST! You are {} down.", bet);
        pounds = pounds - bet;
        return pounds
    } else if dealer > 21 {
        println!("You win! The dealer has gone bust. You gain {} pounds.", bet);
        pounds = pounds + bet;
        return pounds
    } else if total < dealer {
        println!("You have lost ({}) to the dealer ({}). You lose {} pounds.", total, dealer, bet);
        pounds = pounds - bet;
        return pounds
    } else if total > dealer {
        println!("You have won with a score of {}. Dealer has {}. You win {} pounds.", total, dealer, bet);
        pounds = pounds + bet;
        return pounds
    } else if total == dealer {
        println!("You have drawn with the dealer. {} points.", total);
        return pounds
    } else {
        println!("Else statements in WIN/LOSE have gone wrong.");
        return pounds
    }
}

fn input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line.");
    return input.trim().to_string()
}

fn input_int() -> u32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line.");
    let resutl = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => 2,
    };
    return resutl;
}
