use actix_web::{get, Responder, web::Data, Result};
use r2d2_mysql::mysql::prelude::Queryable;
use crate::DbPool;

#[get("/db")]
pub async fn db_version(pool: Data<DbPool>) -> Result<impl Responder> {
    let mut conn = pool.get().unwrap();
    let a: Vec<String> = conn.query("select version();").unwrap();

    Ok(a.get(0).unwrap().clone())
}