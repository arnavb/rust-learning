extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Welcome to the guessing game!");
    println!("Try and guess the random number between 1 and 100 inclusive.");

    let random_number = rand::thread_rng().gen_range(1, 101);


    let mut guess_count = 0;

    loop {
        println!("\nYour guess:");

        let mut user_guess = String::new();

        io::stdin().read_line(&mut user_guess)
            .expect("Failed to read line!");

        guess_count += 1;

        let user_guess: u32 = match user_guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number!");
                continue;
            }
        };

        match user_guess.cmp(&random_number) {
            Ordering::Less => println!("Your number is too small!"),
            Ordering::Greater => println!("Your number is too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }

    println!("You guessed the number in {} attempt(s)!", guess_count);
}
