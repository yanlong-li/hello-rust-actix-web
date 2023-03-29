use actix_web::{get, Responder, HttpResponse};
use actix_redis::{Command, resp_array, RespValue};
use actix_web::web::Data;
use crate::RedisPool;

#[get("/redis")]
pub async fn redis(redis_client: Data<RedisPool>) -> impl Responder {
    let result = redis_client.send(Command(resp_array!["incr","a"])).await;

    let a = match result {
        Ok(result) => match result {
            Ok(RespValue::SimpleString(s)) => format!("SimpleString: {}", s),
            Ok(RespValue::BulkString(s)) => format!("BulkString: {:?}", s),
            Ok(RespValue::Integer(i)) => format!("Integer: {}", i),
            Ok(RespValue::Nil) => format!("Nil"),
            Ok(RespValue::Array(a)) => format!("Array: {:?}", a),
            Err(e) => format!("Error: {:?}", e),
            _ => format!("Error"),
        },
        _ => format!("Error"),
    };
    HttpResponse::Ok().body(a)
}