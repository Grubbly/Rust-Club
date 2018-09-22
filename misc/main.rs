extern crate rand;

use std::io;

fn main() {
    println!("Guess the number");

    // Get input from the user
    let mu guess = String::new();
    io::stdin().read_line(&mut guess)
        .expect("Failed to read from stdin");

    // Convert input to an integer
    let int_guess = guess.trim().parse::<i32>()
        .expect("Failed to parse int");

    println!("You guessed: {}", int_guess);
}