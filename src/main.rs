use std::sync::{Mutex};
use actix::Addr;
use actix_redis::RedisActor;
use actix_web::{App, HttpResponse, HttpServer, Responder, get};
use actix_web::web::Data;
use r2d2::Pool;
use r2d2_mysql::mysql::{Opts, OptsBuilder};
use r2d2_mysql::{MySqlConnectionManager};

mod services;
mod model;

type DbPool = Pool<MySqlConnectionManager>;
type RedisPool = Addr<RedisActor>;


#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let counter = Data::new(model::AppStateWithCounter {
        counter: Mutex::new(0),
    });

    let opts = Opts::from_url("mysql://192.168.2.245:3306?db_name=mysql&user=root&password=123456").unwrap();
    let builder = OptsBuilder::from_opts(opts);
    let manager = MySqlConnectionManager::new(builder);
    let pool: DbPool = Pool::builder().build(manager).expect("connect mysql failed!");

    let redis_client: RedisPool = RedisActor::start("192.168.2.245:6379");

    println!("start web service...");

    let server = HttpServer::new(move || {
        App::new()
            .app_data(counter.clone())
            .app_data(Data::new(pool.clone()))
            .app_data(Data::new(redis_client.clone()))
            .service(hello)
            .configure(services::cfg)
    })
        .bind(("127.0.0.1", 8080));

    let server = match server {
        Ok(server) => {
            println!("web service started!{}", "http://127.0.0.1:8080");
            server
        }
        Err(e) => {
            println!("web service start failed! \n {}", e);
            return Ok(());
        }
    };
    server.run()
        .await
}
