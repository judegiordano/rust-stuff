use dotenv::dotenv;
use rusqlite::Connection;
use std::env;

pub struct Database {
    pub connection: Connection,
}

impl Database {
    pub fn connect() -> Self {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("env DATABASE_URL not set");
        let connection = Connection::open(database_url).expect("error connecting database");
        Database { connection }
    }
}
