use actix_web::{post, web, HttpResponse, Responder};
use serde_json::json;
use sqlx::SqlitePool;

use super::super::database::*;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct LoginBody {
    username: String,
    password: String,
}

#[post("/login")]
pub async fn login(
    pool: web::Data<SqlitePool>,
    json: web::Json<LoginBody>,
) -> actix_web::Result<impl Responder> {
    let user = users::User::fetch_from_username(pool.get_ref(), &json.username).await?;

    let user = match user {
        Some(user) => user,
        None => {
            return Ok(HttpResponse::NotFound().json(json!({
                "msg": "Couldn't find user with that username"
            })));
        }
    };

    if !user.verify_password(&json.password) {
        return Ok(HttpResponse::Forbidden().json(json!({
            "msg": "Invalid password"
        })));
    }

    let token = user.create_login_token(pool.get_ref()).await?;

    Ok(HttpResponse::Ok().json(json!({ "token": token })))
}
