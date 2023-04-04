use crate::schema::User;
use actix_web::web::Json;
use actix_web::{delete, get, post, web, web::Data, Responder, Result};
use sqlx::{MySql, MySqlPool};

#[get("/users")]
pub async fn get_list(pool: Data<MySqlPool>) -> Result<impl Responder> {
    let users = sqlx::query_as::<MySql, User>("SELECT * FROM users")
        .fetch_all(pool.get_ref())
        .await
        .unwrap();
    Ok(Json(users))
}

#[get("/users/{username}")]
pub async fn get_user(
    pool: Data<MySqlPool>,
    username: web::Path<String>,
) -> Result<impl Responder> {
    let username: String = username.parse().unwrap();

    let user = sqlx::query_as::<MySql, User>("SELECT * FROM users WHERE username = ?")
        .bind(username)
        .fetch_one(pool.get_ref())
        .await
        .unwrap();
    Ok(Json(user))
}

#[delete("/users/{username}")]
pub async fn delete_user(
    pool: Data<MySqlPool>,
    username: web::Path<String>,
) -> Result<impl Responder> {
    let username: String = username.parse().unwrap();

    let result = sqlx::query("DELETE FROM users WHERE username = ?")
        .bind(username)
        .execute(pool.get_ref())
        .await
        .unwrap();
    Ok(format!("delete {} row", result.rows_affected()))
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct AddUser {
    username: String,
    password: String,
}

#[post("/users")]
pub async fn add_user(pool: Data<MySqlPool>, data: Json<AddUser>) -> Result<impl Responder> {
    let result = sqlx::query("INSERT INTO users (username, password) VALUES (?, ?)")
        .bind(&data.username)
        .bind(&data.password)
        .execute(pool.get_ref())
        .await
        .unwrap();

    let user = sqlx::query_as::<MySql, User>("SELECT * FROM users WHERE id = ?")
        .bind(result.last_insert_id())
        .fetch_one(pool.get_ref())
        .await
        .unwrap();

    Ok(Json(user))
}
