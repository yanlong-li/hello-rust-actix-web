use actix_web::{HttpResponse, Responder, web, post};
use crate::services::mysql::add_user;

mod user;
mod ws;
mod post;
mod static_files;
mod redis;
mod mysql;

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

pub fn cfg(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .configure(post::cfg)
            .service(echo)
            .service(web::scope("/user").default_service(web::route().to(user::user)))
            .service(ws::ws_index)
            .service(static_files::static_files)
            // .service(counter::counter_plus)
            .service(redis::redis)
            .service(mysql::get_user)
            .service(add_user)
    );
}