#[macro_use]
extern crate text_io;

use std::io;

fn main() {
    let mut users_input = String::new();
    println!("Type a string:");
    io::stdin().read_line(&mut users_input).expect("Failed to read line.");

    println!("You typed: {}", users_input);

    println!("Type an int:");
    // read until a whitespace and try to convert what was read into an i32
    let i: i32 = read!();
    println!("You typed in: {}", i);

    let i_multiplied = i * 3;
    println!("Multiplied by 3= {}", i_multiplied);
}
