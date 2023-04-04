use crate::services::state;
use actix::Addr;
use actix_redis::RedisActor;
use actix_web::web::Data;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use sqlx::MySqlPool;
use std::env;
use std::env::VarError;
use std::sync::Mutex;

mod schema;
mod services;

type RedisPool = Addr<RedisActor>;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    let opts = env::var("DATABASE_URL").unwrap();
    let listen_addr: String = env::var("LISTEN_ADDR")
        .or_else(|_| Ok::<String, VarError>("127.0.0.1".to_string()))
        .unwrap();
    let listen_port: u16 = env::var("LISTEN_PORT")
        .or_else(|_| Ok::<String, VarError>("8080".to_string()))
        .unwrap()
        .parse()
        .unwrap();

    let pool = MySqlPool::connect(&opts).await.unwrap();

    let redis_client: RedisPool = RedisActor::start("192.168.2.245:6379");

    let counter = web::Data::new(state::WebState {
        counter: Mutex::new(0),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .app_data(Data::new(redis_client.clone()))
            .app_data(counter.clone())
            .service(hello)
            .configure(services::cfg)
    })
    .bind((listen_addr.clone(), listen_port))
    .and_then(|op| {
        println!(
            "start web service success http://{}:{}",
            listen_addr, listen_port
        );
        Ok(op)
    })?
    .run()
    .await
}
