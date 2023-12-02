use dotenv::dotenv;
use sha3::Digest;
use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};
use std::{env, process};

use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;

use backend::database::users;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let db_url = &env::var("DATABASE_URL").unwrap();

    if !Sqlite::database_exists(db_url).await.unwrap_or(false) {
        panic!("No databse found, create one with the sqlx cli tool");
    }

    let db = SqlitePool::connect(db_url).await.unwrap();

    let mut rl = DefaultEditor::new().unwrap();
    let readline = rl.readline("Username: ");
    let username = match readline {
        Ok(line) => line,
        Err(ReadlineError::Interrupted) => {
            print!("Exit");
            process::exit(0);
        }
        Err(err) => {
            panic!("Error: {:?}", err);
        }
    };
    let readline = rl.readline("Password: ");
    let password = match readline {
        Ok(line) => line,
        Err(ReadlineError::Interrupted) => {
            print!("Exit");
            process::exit(0);
        }
        Err(err) => {
            panic!("Error: {:?}", err);
        }
    };

    users::User::create(&db, &username, &password)
        .await
        .unwrap();
}
