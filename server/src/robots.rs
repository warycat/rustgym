use super::context::*;
use actix_web::get;
use actix_web::Error;
use actix_web::HttpResponse;

#[get("/robots.txt")]
pub async fn robots_txt() -> Result<HttpResponse, Error> {
    RobotsContext::new().render_wrapper()
}
