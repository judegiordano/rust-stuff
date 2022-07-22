// use actix_web::web::scope;
use actix_web::web::ServiceConfig;
use actix_web::{get, HttpResponse, Responder};
use actix_web::{web::scope, App, HttpServer};
use chrono::offset::Local;
use cuid::cuid;
use dotenv::dotenv;
use rusqlite::{Connection, Result as SqlResult};
use serde::Deserialize;
use serde::Serialize;
use std::env;
use std::io::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct Foo {
    id: String,
    message: String,
}

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

#[allow(dead_code)]
pub struct Migrations;
#[allow(dead_code)]
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
        db.execute("CREATE INDEX idx_message ON foo (message)", ())?;
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
                message: format!(
                    "some_message_{}",
                    Local::now().timestamp_subsec_micros().to_string()
                ),
            };
            let done = transaction.execute(sql, [&entry.id, &entry.message])?;
            affected_rows += done
        }
        transaction.commit()?;
        Ok(affected_rows)
    }
}

pub fn select_all(db: &Connection) -> SqlResult<Vec<Foo>> {
    let mut stmt = db.prepare("SELECT * FROM foo")?;
    let foo_iter = stmt.query_map([], |row| {
        Ok(Foo {
            id: row.get(0)?,
            message: row.get(1)?,
        })
    })?;
    let results = foo_iter.map(|doc| doc.unwrap()).collect::<Vec<Foo>>();
    Ok(results)
}

#[allow(dead_code)]
pub struct AppState {
    db: Database,
}

#[get("")]
pub async fn read() -> impl Responder {
    let db = Database::connect();
    let results = select_all(&db.connection).unwrap();
    HttpResponse::Ok().json(results)
}

pub fn sqlite_router(cfg: &mut ServiceConfig) {
    cfg.service(scope("/read").service(read));
}

pub async fn prepare_server() -> Result<()> {
    // let mut db = Database::connect();
    // Migrations::up(&db.connection);
    // Migrations::seed(&mut db.connection, 10_000);

    let app = move || {
        App::new()
            // .app_data(data)
            .service(scope("/api").configure(sqlite_router))
    };
    HttpServer::new(app).bind(("127.0.0.1", 3000))?.run().await
}
