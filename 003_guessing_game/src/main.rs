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

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
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
