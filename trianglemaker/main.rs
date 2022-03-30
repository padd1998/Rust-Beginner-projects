use std::io;

fn main() {
    println!("Input your number.");
    let mut guess = String::new();
    io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line.");
    let num: i32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => -1,
    };
    let mut row = Vec::new();
    if num < 1 {
        println!("Error. Input must be at least one.")
    } else {
        let mut num2 = num.clone();
        for _i in 0..num {
            for _i in 0..num2 {
                let asterisk = String::from("*");
                row.push(asterisk)
            }
            let row2 = row.join("");

            println!("{:?}", row2);
            row.clear(); // clears vector. 
            num2 -= 1;
        }
    }
}
