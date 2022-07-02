use std::error::Error;
use std::fs::*;

use crate::grep::types::structs::Config;

pub fn get_file_contents(config: &Config) -> Result<(), Box<dyn Error>> {
    let contents: String = read_to_string(String::from(&config.filename))?;
    for line in search(&config.query, &contents) {
        println!("{}", line);
    }
    Ok(())
}

pub fn search<'a>(query: &'a str, content: &'a str) -> Vec<&'a str> {
    let mut matches: Vec<&str> = vec![];
    for line in content.lines() {
        if line.contains(query) {
            matches.push(&line)
        }
    }
    println!("{:#?}", matches);
    matches
}
