use super::app_data::AppData;
use super::context::HomeContext;
use actix_web::get;
use actix_web::web;
use actix_web::Error;
use actix_web::HttpResponse;

#[get("/")]
pub async fn home(data: web::Data<AppData>) -> Result<HttpResponse, Error> {
    let template = HomeContext {
        title: "RustGym",
        tag: &data.tag.borrow(),
    };
    let body = template.render_wrapper()?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}
