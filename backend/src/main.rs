use actix_web::{error, get, middleware, post, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use sqlx::{migrate::MigrateDatabase, Row, Sqlite, SqlitePool};
use std::env;

use backend::routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let db_url = &env::var("DATABASE_URL").expect("There is no DATABASE_URL env variable");
    let pool = SqlitePool::connect(db_url)
        .await
        .expect("Couldn't connect to the sqlite db");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(routes::login::login)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
