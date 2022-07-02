use std::error::Error;
use std::fs::*;

use crate::grep::types::structs::Config;

pub fn get_file_contents(config: &Config) -> Result<(), Box<dyn Error>> {
    let contents: String = read_to_string(String::from(&config.filename))?;
    let count = search(&config.query, &contents).len();
    for line in search(&config.query, &contents) {
        println!("{}", line);
    }
    println!("matches: {:#?}", count);
    Ok(())
}

pub fn search<'a>(query: &'a str, content: &'a str) -> Vec<&'a str> {
    let mut matches: Vec<&str> = vec![];
    for line in content.lines() {
        if line.contains(query) {
            matches.push(line)
        }
    }
    matches
}
