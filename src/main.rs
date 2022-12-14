use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    guess_the_number();
}

fn guess_the_number() {
    println!("*****Guess the number!**********");

    let magic_number = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut guess = String::new();
        println!("{guess}");
        println!("Enter your guess: ");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("The magic number is {magic_number}");
        println!("Your guess was {guess}");

        match guess.cmp(&magic_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You got it!");
                break;
            }
        }
    }
}
