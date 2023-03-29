use std::path::PathBuf;
use actix_files::NamedFile;
use actix_web::{HttpRequest, Result, get};

#[get("/static/{filename:.*}")]
pub async fn static_files(req: HttpRequest) -> Result<NamedFile> {
    let path: PathBuf = format!("static/{}", req.match_info().query("filename").replace("../", "")).parse().unwrap();
    Ok(NamedFile::open(path)?)
}