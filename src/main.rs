use std::env;
use actix::Addr;
use actix_redis::RedisActor;
use actix_web::{App, get, HttpResponse, HttpServer, Responder};
use actix_web::web::Data;
use sqlx::{MySqlPool};

mod services;
mod schema;

type RedisPool = Addr<RedisActor>;


#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    let opts = env::var("DATABASE_URL").unwrap();
    let listen_addr: String = env::var("LISTEN_ADDR").unwrap();
    let listen_port: u16 = env::var("LISTEN_PORT").unwrap().parse().unwrap();

    let pool = MySqlPool::connect(&opts).await.unwrap();

    let redis_client: RedisPool = RedisActor::start("192.168.2.245:6379");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .app_data(Data::new(redis_client.clone()))
            .service(hello)
            .configure(services::cfg)
    })
        .bind((listen_addr.clone(), listen_port)).and_then(|op| {
        println!("start web service success http://{}:{}", listen_addr, listen_port);
        Ok(op)
    })?
        .run()
        .await
}
