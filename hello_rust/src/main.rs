use std::io;
use rand::prelude::*;
use std::cmp::Ordering;

fn main() {
    println!("Lets do a guessing game");
    println!("Take a guess at the secret number");

    // Let is for creating a variable
    // Variables are immutable by standard so 'mut' allows the variable to be mutable
    let the_number = rand::rng().random_range(1..=10);

    

    // Fairly standard way of reading in some data from the user
    // Can wirte this on one line but rust allows for multiple so we will use it
    

    

    loop {
        let mut guess = String::new();
        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

        let guess: u32 = guess.trim().parse().expect("Please type a number");
        println!("You guessed {guess}");

        match guess.cmp(&the_number){
        Ordering::Less => println!("Too small"),
        Ordering::Greater => println!("Too big"),
        Ordering::Equal => {
            println!("Correct");
            break;
        }
    }
    }
    
    

}
