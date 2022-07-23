use chrono::Local;
use cuid::cuid;
use rusqlite::{Connection, Result as SqlResult};

use crate::models::Foo;

pub struct Migrations;
impl Migrations {
    pub fn up(db: &Connection) -> SqlResult<usize> {
        // id    INTEGER PRIMARY KEY AUTOINCREMENT,
        let affected_rows = db.execute(
            "CREATE TABLE IF NOT EXISTS foo (
                id    TEXT PRIMARY KEY,
                message TEXT
            )",
            (),
        )?;
        db.execute(
            "CREATE INDEX IF NOT EXISTS idx_message ON foo (message)",
            (),
        )?;
        Ok(affected_rows)
    }

    pub fn truncate(db: &Connection) -> SqlResult<usize> {
        let affected_rows = db.execute("TRUNCATE TABLE foo", ())?;
        Ok(affected_rows)
    }

    pub fn down(db: &Connection) -> SqlResult<usize> {
        let affected_rows = db.execute("DROP TABLE foo", ())?;
        db.execute("DROP INDEX idx_message", ())?;
        Ok(affected_rows)
    }

    pub fn seed(db: &mut Connection, amount: u32) -> SqlResult<usize> {
        let mut affected_rows = 0;
        let transaction = db.transaction()?;
        let sql = "INSERT INTO foo (id, message) VALUES (?1, ?2)";
        for _ in 1..=amount {
            let entry = Foo {
                id: cuid().unwrap(),
                message: format!("some_message_{}", Local::now().timestamp_subsec_micros()),
            };
            let done = transaction.execute(sql, [&entry.id, &entry.message])?;
            affected_rows += done
        }
        transaction.commit()?;
        Ok(affected_rows)
    }
}
