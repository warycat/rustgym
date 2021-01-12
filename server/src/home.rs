use super::app_data::AppData;
use super::context::*;
use actix_web::get;
use actix_web::web;
use actix_web::Error;
use actix_web::HttpRequest;
use actix_web::HttpResponse;
use rand::thread_rng;
use rand::Rng;
use rustgym_characters::CHARACTERS;

#[get("/")]
pub async fn home(req: HttpRequest, data: web::Data<AppData>) -> Result<HttpResponse, Error> {
    let n = CHARACTERS.len();
    let k = thread_rng().gen::<usize>();
    HomeContext::new(
        AppContext::new(data),
        req.path().to_string(),
        CHARACTERS[k % n].to_string(),
    )
    .render_wrapper()
}
