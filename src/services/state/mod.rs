use actix_web::{get, web, HttpResponse, Responder, Result};
use std::sync::Mutex;

pub struct WebState {
    pub counter: Mutex<usize>,
}

#[get("/state")]
pub async fn get_state(data: web::Data<WebState>) -> Result<impl Responder> {
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;
    Ok(HttpResponse::Ok().body(format!("counter: {}", counter)))
}
