use rand::Rng;
use sqlx::{Error::RowNotFound, SqlitePool};
use std::error::Error;

use super::super::hash_password;

#[derive(Debug, sqlx::FromRow, serde::Serialize, serde::Deserialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub password_hash: String,
}

fn fetch_one<T>(res: Result<T, sqlx::error::Error>) -> Result<Option<T>, Box<dyn Error>> {
    match res {
        Ok(ok) => Ok(Some(ok)),
        Err(err) => match err {
            RowNotFound => Ok(None),
            _ => Err(err.into()),
        },
    }
}

impl User {
    pub fn verify_password(&self, password: &str) -> bool {
        self.password_hash == hash_password(password)
    }

    pub async fn fetch_from_username(
        pool: &SqlitePool,
        username: &str,
    ) -> Result<Option<User>, Box<dyn Error>> {
        let user_res = sqlx::query_as!(User, r#"SELECT * FROM users WHERE username = ?"#, username)
            .fetch_one(pool)
            .await;

        fetch_one::<User>(user_res)
    }

    pub async fn create_login_token(&self, pool: &SqlitePool) -> Result<String, Box<dyn Error>> {
        let mut rng = rand::thread_rng();

        const LETTER_POOL: &'static str =
            "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

        let token = (0..250)
            .map(|_| rng.gen_range(0..LETTER_POOL.len()))
            .map(|i| LETTER_POOL.chars().nth(i).unwrap())
            .collect::<String>();

        sqlx::query!(
            r#"
             INSERT INTO
                login_tokens
            VALUES
                (null, ?, ?)
             "#,
            token,
            self.id
        )
        .execute(pool)
        .await?;

        Ok(token)
    }

    pub async fn create(
        pool: &SqlitePool,
        username: &str,
        password: &str,
    ) -> Result<(), Box<dyn Error>> {
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

    pub async fn fetch_from_token(
        pool: &SqlitePool,
        token: &str,
    ) -> Result<Option<User>, Box<dyn Error>> {
        let user_res = sqlx::query_as!(
            User,
            r#"SELECT
                users.id, users.username, users.password_hash
            FROM
                login_tokens
            INNER JOIN users ON
                users.id = login_tokens.user_id
            WHERE
                login_tokens.id = ?"#,
            token
        )
        .fetch_one(pool)
        .await;

        fetch_one::<User>(user_res)
    }
}
