use crate::app_data::AppData;
use crate::context::*;
use crate::session_data::update_session;
use actix_session::Session;
use actix_web::get;
use actix_web::web;
use actix_web::Error;
use actix_web::HttpRequest;
use actix_web::HttpResponse;

#[get("/find")]
pub async fn find(
    req: HttpRequest,
    data: web::Data<AppData>,
    session: Session,
) -> Result<HttpResponse, Error> {
    let session_data = update_session(session)?;
    FindContext::new(AppContext::new(data), session_data, req.path().to_string()).render_wrapper()
}
