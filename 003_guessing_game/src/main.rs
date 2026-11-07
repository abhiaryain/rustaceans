use rand::prelude::*;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!();
    println!("=====================================");
    println!("          GUESS THE NUMBER!          ");
    println!("=====================================");
    println!();

    let secret_number = rand::rng().random_range(1..=100);

    let mut guesses = 0;

    loop {
        guesses += 1;

        println!("Please input your guess:");

        let mut guess = String::new(); // The :: syntax in the ::new line indicates that new is an associated function of the String type. An associated function is a function that’s implemented on a type, in this case String
        // `let mut guess = String::new();` line has created a mutable variable that is currently bound to a new, empty instance of a String.
        io::stdin()
            .read_line(&mut guess) // `read_line` method accept `&mut guess` means mutable reference and return `Return` enum variant.
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid number entered, please try again!");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Your guess {} is too small!", guess),
            Ordering::Greater => println!("Your guess {} is too big!", guess),
            Ordering::Equal => {
                println!(
                    "Congratulations! You guessed the secret number {} in {} tries!",
                    secret_number, guesses
                );
                break;
            }
        }
    }
}
