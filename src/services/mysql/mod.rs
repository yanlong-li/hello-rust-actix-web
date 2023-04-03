use actix_web::{get, Responder, web::Data, Result, web, post};
use actix_web::web::Json;
use sqlx::{MySql, MySqlPool};
use crate::schema::User;


#[get("/db/{username}")]
pub async fn get_user(
    pool: Data<MySqlPool>,
    username: web::Path<String>,
) -> Result<impl Responder> {
    let username: String = username.parse().unwrap();

    let user = sqlx::query_as::<MySql, User>("SELECT * FROM users WHERE username = ?")
        .bind(username)
        .fetch_one(pool.get_ref())
        .await.unwrap();
    Ok(Json(user))
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct AddUser {
    username: String,
    password: String,
}

#[post("/db")]
pub async fn add_user(
    pool: Data<MySqlPool>,
    data: Json<AddUser>,
) -> Result<impl Responder> {
    let result = sqlx::query("INSERT INTO users (username, password) VALUES (?, ?)")
        .bind(&data.username).bind(&data.password)
        .execute(pool.get_ref())
        .await.unwrap();

    let user = sqlx::query_as::<MySql, User>("SELECT * FROM users WHERE id = ?")
        .bind(result.last_insert_id())
        .fetch_one(pool.get_ref())
        .await.unwrap();

    Ok(Json(user))
}