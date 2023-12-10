use std::io::{self};

use std::collections::HashMap;

use rand::prelude::*;

fn get_input() -> String {
    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    return input;
}

fn generate_fizz_buzz() -> String {
    let array_phases: [&str; 3] = ["fizz", "buzz", "fizzBuzz"];
    let mut rnd = rand::thread_rng();

    return array_phases[rnd.gen_range(0..3)].to_string();
}

fn calculate_fizz_buzz() {
    println!("Calculate fizz buzz\n");

    let mut dict: HashMap<String, i16> = HashMap::new();

    for i in 1..10 {
        let phase: String = generate_fizz_buzz();
        println!("index: {i}, phase: {phase}");
        let entry = dict.entry(phase).or_insert(0);
        *entry += 1;
    }

    println!("\nResults:");
    for (key, value) in &dict {
        println!("{}: {}", key, value);
    }
}

fn fizz_buzz() {
    println!("Please enter the number");
    let input = get_input();

    let calculation_result: String = match input.trim().parse::<i32>() {
        Ok(n) => {
            if n % 15 == 0 {
                "FizzBuzz".to_string()
            } else if n % 3 == 0 {
                "Fizz".to_string()
            } else if n % 5 == 0 {
                "Buzz".to_string()
            } else {
                "Entered some another value".to_string()
            }
        },
        Err(_) => "Invalid input. Please enter a valid integer.".to_string()
    };

    println!("Result is: {:?}", calculation_result);
}

fn fizz_or_buzz() {
    println!("Please enter the number");
    let input: String = get_input();
    let comprasion_result: String = match input.trim().parse::<i32>() {
        Ok(n) => {
            if n % 3 == 0 && n % 5 != 0 {
                "Fizz but not Buzz".to_string()
            } else if n % 5 == 0 && n % 3 != 0 {
                "Buzz but not Fizz".to_string()
            } else {
                "Not Fizz or Buzz".to_string()
            }
        }
        Err(_) => "Invalid input. Please enter a valid integer.".to_string()
    };

    println!("Result is: {:?}", comprasion_result);
}

fn main() {
    println!("Select fizz buzz:");
    println!("1. calculate fizz buzz");
    println!("2. fizz buzz");
    println!("3. Fizz or Buzz");
    let mut input_user: String = String::new();
    io::stdin()
        .read_line(&mut input_user)
        .expect("Failed to read line");

    match input_user.trim() {
        "1" => calculate_fizz_buzz(),
        "2" => fizz_buzz(),
        "3" => fizz_or_buzz(),
        _ => println!("\nEntered something another value\n")
    }
}
