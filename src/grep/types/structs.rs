use std::env;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("correct usage is: program.exe <text_slice> <filename>");
        }
        let query: String = String::from(&args[1]);
        let filename: String = String::from(&args[2]);
        let ignore_case: bool = env::var("IGNORE_CASE").is_ok();
        Ok(Config {
            query,
            filename,
            ignore_case,
        })
    }
}
