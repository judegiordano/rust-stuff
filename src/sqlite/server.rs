use actix_web::web::ServiceConfig;
use actix_web::{get, HttpResponse, Responder};
use actix_web::{web::scope, App, HttpServer};
use rusqlite::{Connection, Result as SqlResult};
use std::io::Result;

use crate::db::Database;
use crate::models::Foo;

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
    let app = move || {
        App::new()
            // .app_data(data)
            .service(scope("/api").configure(sqlite_router))
    };
    println!("server listening on http://127.0.0.1:3000");
    HttpServer::new(app).bind(("127.0.0.1", 3000))?.run().await
}
