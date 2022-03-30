use std::fs::File;
use std::io;
use std::io::{BufReader, BufRead};
// use std::error::Error;

fn total_question(question_num: usize, mut score: usize) -> usize {
    let question_pro = read_file_line("quiz.txt", question_num - 1);
    let question:String = match question_pro {
        Ok(ans) => ans,
        Err(_) => String::from("Who knows."),
    };
    println!("{}", question);
    let inpu = input();
    // 2nd input should be line of quiz.txt
    if check_answer(&inpu, question_num) {
        if score == 0 {
            score += 1;
            println!("Good answer! You have {} point.", score);
        } else {
            score += 1;
            println!("Good answer! You have {} points.", score);
        }
    } else {
        if score == 0 {
            println!("Wrong! You have {} point.", score);
        } else {
            println!("Wrong! You have {} points.", score);
        }
    }
    return score;
}

fn main() {
    println!("Hello, world! Welcome to my quiz.");
    
    let mut _score = 0; 
    _score = total_question(1, _score);
    _score = total_question(3, _score);
    _score = total_question(5, _score);
    _score = total_question(7, _score);
}

fn check_answer(input: &str, num: usize) -> bool { 
    let compare_1 = match read_file_line("quiz.txt", num) {
        Ok(ans) => ans,
        Err(_) => String::from("Who knows."),
    };
    let compare = compare_1.trim().to_string();
    
    if input.trim().to_string() == compare {
        return true;
        
    } else if input.trim().to_string() != compare {
        return false;
    } else {
        println!("Neither boolean is triggered.");
        return true;
    }
}

fn read_file_line(file: &str, num: usize) -> Result<String, Box<dyn std::error::Error>> {
    let file = File::open(file)?;
    let reader = BufReader::new(file);
    let mut contents = Vec::new();
    for line in reader.lines() {
        contents.push(line?);
    }
    //println!("{:?}", contents);
    let element = &contents[num];
    Ok(element.to_string())
}

fn input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input.");
    return input;
}
