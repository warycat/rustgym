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

#[get("/adventofcode/{id}")]
async fn adventofcode_detail(
    data: web::Data<AppData>,
    req: HttpRequest,
    web::Path(id_): web::Path<i32>,
    pool: web::Data<SqlitePool>,
    session: Session,
) -> Result<HttpResponse, Error> {
    let session_data = update_session(session)?;
    use rustgym_schema::schema::adventofcode_description::dsl::*;
    use rustgym_schema::schema::adventofcode_solution::dsl::*;
    let conn = conn(pool)?;
    let description = adventofcode_description
        .find(id_)
        .first(&conn)
        .map_err(ErrorNotFound)?;
    let solution = adventofcode_solution
        .find(id_)
        .first(&conn)
        .map_err(ErrorNotFound)?;
    AdventOfCodeDetailContext::new(
        AppContext::new(data),
        session_data,
        req.path().to_string(),
        description,
        solution,
    )
    .render_wrapper()
}
