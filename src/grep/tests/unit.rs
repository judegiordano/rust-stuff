#![allow(dead_code)]
#![allow(unused_imports)]
//
use crate::grep::lib::util::*;

#[cfg(test)]
pub mod tests {
    pub use super::*;

    #[test]
    fn one_result() {
        let query: &str = "duct";
        let contents: &str = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
