use std::io;

fn main() {
    println!("Guess a random number");
    let mut guess= String::new();
    io::stdin().read_line(&mut guess).expect("Failed to Readline");
    println!("{guess}");
}
