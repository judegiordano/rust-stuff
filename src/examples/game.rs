use std::cmp::Ordering::{Equal, Greater, Less};
use std::io::stdin as input;

use crate::utils::random::random_number;

// constants can be declared in global scope
const LOWER_BOUND: u32 = 0;
const UPPER_BOUND: u32 = 100;

pub fn init_game() {
    let secret_number: u32 = random_number(LOWER_BOUND, UPPER_BOUND);
    println!("guess the number!");

    loop {
        println!("input your guess");
        let mut guess: String = String::new();

        input().read_line(&mut guess).expect("Failed to read line");

        // shadow original variable with 'turbofish' syntax
        let guess: u32 = match guess.trim().parse::<u32>() {
            Ok(num) => num,
            Err(_) => {
                println!("input a number between 1 and 100");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            // each 'arm' of the match condition
            Less => println!("The number is higher!"),
            Greater => println!("The number is lower!"),
            Equal => {
                println!("You win!");
                break;
            }
        };
    }
}
