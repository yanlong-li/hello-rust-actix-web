use actix_web::http::StatusCode;
use actix_web::{post, web, HttpResponse, Responder, Result};

mod post;
mod redis;
pub mod state;
mod static_files;
mod users;
mod ws;

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

pub fn cfg(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .configure(post::cfg)
            .service(echo)
            .service(ws::ws_index)
            .service(static_files::static_files)
            .service(redis::redis)
            .service(users::get_user)
            .service(users::add_user)
            .service(users::get_list)
            .service(users::delete_user)
            .service(state::get_state)
            .default_service(web::route().to(not_found)),
    );
}

async fn not_found() -> Result<impl Responder> {
    Ok(HttpResponse::build(StatusCode::NOT_FOUND).body("PageNotFound"))
}
