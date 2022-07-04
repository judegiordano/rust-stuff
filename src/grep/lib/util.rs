use std::error::Error;
use std::fs::*;

use crate::grep::types::structs::Config;

pub fn get_file_contents(config: &Config) -> Result<(), Box<dyn Error>> {
    let contents: String = read_to_string(String::from(&config.filename))?;
    let results: Vec<&str> = search(&config.query, &contents, config.ignore_case);
    let count: usize = results.len();
    for line in results {
        println!("{}", line);
    }
    println!("matches: {:#?}", count);
    Ok(())
}

pub fn search<'a>(query: &'a str, content: &'a str, ignore_case: bool) -> Vec<&'a str> {
    if ignore_case {
        let query: String = query.to_lowercase();
        return content
            .lines()
            .filter(|line| line.to_lowercase().contains(&query))
            .collect();
    }
    content
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}
