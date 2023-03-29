use actix_web::{HttpResponse, Responder, web, get};
use crate::model;

#[get("/counter")]
pub async fn counter_plus(data: web::Data<model::AppStateWithCounter>) -> impl Responder {
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;
    HttpResponse::Ok().body(format!("counter: {counter}"))
}
