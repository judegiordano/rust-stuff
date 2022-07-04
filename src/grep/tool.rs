use std::env;
use std::process;

use crate::grep::lib::util::*;
use crate::grep::types::structs::*;

/// searches a file for the given query
///
/// ## Arguments
///
/// * `query` - A string slice that holds the text to query
/// * `filename` - A string slice that searches for a file by name
///
/// ---
/// ## Examples
///
/// ```sh
/// # cli usage
/// cargo run frog poem.txt
/// ```
pub fn grep_cli() {
    let config: Config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {:#?}", err);
        process::exit(1);
    });
    println!("searching for slice {:#?}", config.query);
    println!("in file {:#?}", config.filename);
    println!("---------");
    if let Err(e) = get_file_contents(&config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
    println!("---------");
}
