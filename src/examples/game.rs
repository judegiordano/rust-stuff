use std::cmp::Ordering::{Equal, Greater, Less};
use std::io::stdin as input;

use crate::utils::random::random_number;

pub fn init_game() {
    let secret_number: u32 = random_number();
    println!("guess the number!");

    loop {
        println!("input your guess");
        let mut guess: String = String::new();

        input().read_line(&mut guess).expect("Failed to read line");

        // shadow original variable with 'turbofish' syntax
        let guess: u32 = guess.trim().parse::<u32>().expect("Please input a number");

        let result: &str = match guess.cmp(&secret_number) {
            // each 'arm' of the match condition
            Less => "the number is higher!",
            Greater => "the number is lower!",
            Equal => {
                println!("You win!");
                break;
            }
        };

        println!("{result}");
    }
}
