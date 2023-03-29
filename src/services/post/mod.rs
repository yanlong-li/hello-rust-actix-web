use actix_web::{Responder, web, post, Result};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct FormData {
    username: String,
    password: String,
}

#[post("/form-data")]
pub async fn post_data(form: web::Form<FormData>) -> Result<String> {
    Ok(format!("welcome {}!Your password is {}", form.username, form.password))
}

#[post("/json")]
pub async fn post_json(form: web::Json<FormData>) -> Result<impl Responder> {
    Ok(web::Json(form))
}

pub fn cfg(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/post")
            .service(post_data)
            .service(post_json)
    );
}