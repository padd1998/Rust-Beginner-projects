// Moves files from one place to another. 
extern crate fs_extra;
use fs_extra::file::move_file;
use std::io;
use fs_extra::file::CopyOptions;

fn main() {
    println!("Hello, world! Which file do you want to move?");
    let mut from = String::new();
    io::stdin().read_line(&mut from).expect("Failed to read line.");
    let mut to = String::new();
    println!("Where do you want it moved to?");
    io::stdin().read_line(&mut to).expect("Failed to read destination.");
    let options = CopyOptions::new();
    let result = match move_file(from.trim(), to.trim(), &options) {
        Ok(ans) => ans,
        Err(_e) => 0,
    }; 
    println!("{}", result);
}
