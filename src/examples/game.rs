use std::io::stdin as input;

use crate::utils::random::random_number;

pub fn init_game() {
    let magic_number = random_number();
    println!("{magic_number}");
    println!("guess the number!");
    println!("input your guess");
    let mut guess: String = String::new();

    input().read_line(&mut guess).expect("Failed to read line");

    println!("guess: {guess}")
}
