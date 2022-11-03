use rand::Rng;
use std::io;

fn main() {
    guess_the_number();
}

fn guess_the_number() {
    println!("*****Guess the number!**********");

    let magic_number = rand::thread_rng().gen_range(1..=100);
    let mut guess = String::new();
    println!("Enter your guess: ");
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line!");

    println!("The magic number is {magic_number}");
    println!("Your guess was {guess}");
}
