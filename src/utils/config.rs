use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::{
    env::{self, VarError},
    str::FromStr,
};

// constants can be declared in global scope
pub const GLOBAL_VALUE: &str = "sup";

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub rust_backtrace: u32,
}

pub fn parse_env_variable<T: FromStr>(name: &str) -> T {
    // get variable
    let normalized: String = name.trim().to_uppercase().trim().to_string();
    let value: String = env::var(&normalized).unwrap_or_else(|_| panic!("{} not set", &normalized));
    // parse data type
    value
        .parse::<T>()
        .unwrap_or_else(|_| panic!("error parsing environment variable {}", &normalized))
}

impl Config {
    pub fn new() -> Result<Config, VarError> {
        dotenv().ok();
        let variables: Config = Config {
            rust_backtrace: parse_env_variable::<u32>("RUST_BACKTRACE"),
        };
        Ok(variables)
    }
}
