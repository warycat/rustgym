use actix_files::NamedFile;
use actix_web::get;
use actix_web::HttpRequest;
use actix_web::Result;
use std::path::PathBuf;

#[get("/pkg/{filename:.*}")]
async fn client_files(req: HttpRequest) -> Result<NamedFile> {
    let filename = req.match_info().query("filename");
    let path: PathBuf = ["pkg", filename].iter().collect();
    Ok(NamedFile::open(path)?)
}

#[get("/static/{filename:.*}")]
async fn static_files(req: HttpRequest) -> Result<NamedFile> {
    let filename = req.match_info().query("filename");
    let path: PathBuf = ["static", filename].iter().collect();
    Ok(NamedFile::open(path)?)
}

#[get("/data/{filename:.*}")]
async fn data_files(req: HttpRequest) -> Result<NamedFile> {
    let filename = req.match_info().query("filename");
    let path: PathBuf = ["data", filename].iter().collect();
    Ok(NamedFile::open(path)?)
}

#[get("/favicon.ico")]
async fn favicon_file(_req: HttpRequest) -> Result<NamedFile> {
    let path: PathBuf = ["static", "ico", "windows.ico"].iter().collect();
    Ok(NamedFile::open(path)?)
}
