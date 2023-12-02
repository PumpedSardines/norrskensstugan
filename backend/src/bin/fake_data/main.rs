use dotenv::dotenv;
use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};
use std::env;

use backend::database::users;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let db_url = &env::var("DATABASE_URL").unwrap();

    if !Sqlite::database_exists(db_url).await.unwrap_or(false) {
        panic!("No databse found, create one with the sqlx cli tool");
    }

    let db = SqlitePool::connect(db_url).await.unwrap();

    // Add users
    {
        let dummy_users = vec![("admin", "abc123")];

        for dummy_user in dummy_users {
            users::User::create(&db, dummy_user.0, dummy_user.1)
                .await
                .unwrap();
        }
    }
}
