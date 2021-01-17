use actix_files::NamedFile;
use actix_web::get;
use actix_web::HttpRequest;
use actix_web::Result;
use rustgym_consts::*;
use std::path::Path;
use std::path::PathBuf;

#[get("/pkg/{filename:.*}")]
async fn client_files(req: HttpRequest) -> Result<NamedFile> {
    let filename = req.match_info().query("filename");
    let path: PathBuf = ["pkg", filename].iter().collect();
    Ok(NamedFile::open(path)?)
}
