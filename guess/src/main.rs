extern crate rand;
use rand::Rng;

use std::cmp::Ordering;
use std::io;

fn main() {
    
    let num: i32 = rand::thread_rng().gen_range(0,100);
    println!("Guess the number!");

    loop {
        // Get input from the user
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read from stdin");

        // Convert input to an integer
        let guess = match guess.trim().parse::<i32>() {
            
            Ok(guess_num) => {
                guess_num
            },

            Err(_) => {
                println!("BAD");
                continue;
            },

        };


        println!("You guessed: {}", guess);

        // match returns an Ok or Err - use these for error checking
        match guess.cmp(&num) {
            Ordering::Less => println!("Too low!"),
            Ordering::Greater => println!("Too high! 420 Blaze it"),
            Ordering::Equal =>  { 
                println!("You got it!");
                break;
            },
            // _ => println!("You got it"); // <- _ is like a default case in switch statements.
        }


    }
}