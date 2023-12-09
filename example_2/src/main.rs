use std::io::{self};

use std::collections::HashMap;

use rand::prelude::*;

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

fn main() {
    let mut input_user: String = String::new();
    io::stdin()
        .read_line(&mut input_user)
        .expect("Failed to read line");

    match input_user.trim() {
        "1" =>  calculate_fizz_buzz(),
        _ => println!("\nEntered something another value\n")
    }
}
