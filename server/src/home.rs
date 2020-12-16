use super::app_data::AppData;
use super::context::*;
use actix_web::get;
use actix_web::web;
use actix_web::Error;
use actix_web::HttpResponse;

#[get("/")]
pub async fn home(data: web::Data<AppData>) -> Result<HttpResponse, Error> {
    HomeContext::new(AppContext::new(data)).render_wrapper()
}
