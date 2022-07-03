use std::error::Error;
use std::fs::*;

use crate::grep::types::structs::Config;

pub fn get_file_contents(config: &Config) -> Result<(), Box<dyn Error>> {
    let contents: String = read_to_string(String::from(&config.filename))?;
    let results: Vec<&str> = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };
    let count: usize = results.len();
    for line in results {
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

pub fn search_case_insensitive<'a>(query: &'a str, content: &'a str) -> Vec<&'a str> {
    let query: String = query.to_lowercase();
    let mut matches: Vec<&str> = vec![];
    for line in content.lines() {
        if line.to_lowercase().contains(&query) {
            matches.push(line)
        }
    }
    matches
}
