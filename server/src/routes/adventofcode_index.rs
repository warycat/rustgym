use crate::context::*;
use crate::db::*;
use crate::session_data::update_session;
use crate::AppData;
use actix_session::Session;
use actix_web::error::ErrorNotFound;
use actix_web::get;
use actix_web::web;
use actix_web::Error;
use actix_web::HttpRequest;
use actix_web::HttpResponse;
use diesel::prelude::*;

#[get("/adventofcode")]
pub async fn adventofcode_index(
    data: web::Data<AppData>,
    req: HttpRequest,
    pool: web::Data<SqlitePool>,
    session: Session,
) -> Result<HttpResponse, Error> {
    let session_data = update_session(session)?;
    use rustgym_schema::schema::adventofcode_description::dsl::*;
    let conn = conn(pool)?;
    let rows: Vec<AdventOfCodeIndexRow> = adventofcode_description
        .select((id, year, day, title))
        .load(&conn)
        .map_err(ErrorNotFound)?;
    AdventOfCodeIndexContext::new(
        AppContext::new(data),
        session_data,
        req.path().to_string(),
        rows,
    )
    .render_wrapper()
}
