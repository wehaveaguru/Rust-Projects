use std::io;
use std::cmp::Ordering;
use rand::prelude::*;

fn main() {
    println!("Guess a random number");
    let secret_number = rand::rng().random_range(1..=10);
    let mut guess=String::new();
    io::stdin().read_line(&mut guess).expect("Failed to Readline");
    let guess: u32 = guess.trim().parse().expect("please type a numbers");
    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small. The number is {secret_number}"),
        Ordering::Equal => println!("You win"),
        Ordering::Greater => println!("Too big. The number is {secret_number}"),
    }
}
