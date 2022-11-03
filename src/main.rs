use std::io;

fn main() {
    guess_the_number();
}

fn guess_the_number() {
    println!("*****Guess the number!**********");

    let mut guess = String::new();
    println!("Enter your guess: ");
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line!");

    println!("Your guess was {guess}");
}
