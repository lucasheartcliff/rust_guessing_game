use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("Guess the number");
    const MIN: u8 = 1;
    const MAX: u8 = 100;

    let secret_number = rand::thread_rng().gen_range(MIN..=MAX);

    loop {
        println!("Please input your guess");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u8 = match guess.trim().parse() {
            Ok(x) => x,
            Err(_) => continue,
        };

        println!("Your guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }

        println!("The secret number is: {secret_number}");
    }
}
