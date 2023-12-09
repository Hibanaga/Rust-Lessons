use std::io;

use rand::prelude::*;

fn main() {
    let mut rng = rand::thread_rng();
    println!("Guess a number");

    loop {
        let random_number = rng.gen_range(0..10);

        println!("Entered a number: ");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let number = guess.trim().parse::<i32>();

        match number {
            Ok(parsed_number) => {
                if parsed_number == random_number {
                    println!("Hip hip you entered correct value");
                    println!("Entered: {parsed_number}");
                    println!("Expected: {random_number}");
                    break;
                } else {
                    println!("Oh no... you enter wrong value");
                }
            },
            Err(err) => eprintln!("Error: {}", err),
        }
    }
}
