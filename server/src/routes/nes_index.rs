use crate::app_data::AppData;
use crate::context::*;
use crate::db::*;
use crate::session_data::update_session;
use actix_session::Session;
use actix_web::error::ErrorNotFound;
use actix_web::get;
use actix_web::web;
use actix_web::Error;
use actix_web::HttpRequest;
use actix_web::HttpResponse;
use diesel::prelude::*;
use rustgym_schema::schema;

#[get("/nes")]
pub async fn nes_index(
    data: web::Data<AppData>,
    req: HttpRequest,
    pool: web::Data<SqlitePool>,
    session: Session,
) -> Result<HttpResponse, Error> {
    let session_data = update_session(session)?;
    use schema::nes_rom::dsl::*;
    let conn = conn(pool)?;
    let rows: Vec<NesIndexRow> = nes_rom
        .select((id, title, description, image))
        .load(&conn)
        .map_err(ErrorNotFound)?;
    NesIndexContext::new(
        AppContext::new(data),
        session_data,
        req.path().to_string(),
        rows,
    )
    .render_wrapper()
}
