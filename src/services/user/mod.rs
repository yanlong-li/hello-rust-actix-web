use actix_web::{HttpResponse, Responder};

pub async fn user() -> impl Responder {
    HttpResponse::Ok().body(format!("UserController"))
}