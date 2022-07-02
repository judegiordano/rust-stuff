pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("correct usage is: program.exe <text_slice> <filename>");
        }
        let query: String = String::from(&args[1]);
        let filename: String = String::from(&args[2]);
        Ok(Config { query, filename })
    }
}
