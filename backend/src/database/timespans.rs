use rand::Rng;
use sqlx::{Error::RowNotFound, SqlitePool};
use std::error;

use super::super::hash_password;

#[derive(Debug, sqlx::FromRow, serde::Serialize, serde::Deserialize)]
pub struct Timespan {
    pub id: i64,
    pub date_start: Option<String>,
    pub date_end: Option<String>,
    pub r#type: i64,
}

impl Timespan {
    pub async fn fetch_all(pool: &SqlitePool) -> Result<Vec<Timespan>, Box<dyn error::Error>> {
        let timespans = sqlx::query_as!(Timespan, r#"SELECT * FROM timespans"#)
            .fetch_all(pool)
            .await?;

        return Ok(timespans);
    }

    pub async fn create(
        pool: &SqlitePool,
        username: &str,
        password: &str,
    ) -> Result<(), Box<dyn error::Error>> {
        let password = hash_password(password);

        match sqlx::query!(
            r#"
            INSERT INTO users 
            VALUES (null, ?, ?)
             "#,
            username,
            password
        )
        .execute(pool)
        .await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(err.into()),
        }
    }
}

#[derive(Debug)]
enum Error {
    DateStartIsInvalid,
    DateEndIsInvliad,
    TypeIsInvalid,
}

impl error::Error for Error {}
