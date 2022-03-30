use std::io;

fn main() {
    println!("Input the number of terms you want.");
    let mut input = String::new();
    io::stdin()
    .read_line(&mut input)
    .expect("Failed to read line.");
    let input = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => -1,
    };
    if input > 0 {
        let mut num1 = 1;
        let mut num2 = 1;
        for _i in 0..input {
            let value = num1;
            num1 += num2;
            println!("{}", num1);
            num2 = value;
        }
    } else {
        println!("Invalid input.")
    }

}
