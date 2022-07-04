use std::env;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn new(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        // ignore first argument; program name
        args.next();
        let query: String = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };
        let filename: String = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };
        let ignore_case: bool = env::var("IGNORE_CASE").is_ok();
        Ok(Config {
            query,
            filename,
            ignore_case,
        })
    }
}
