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

#[get("/google")]
pub async fn google_index(
    data: web::Data<AppData>,
    req: HttpRequest,
    pool: web::Data<SqlitePool>,
    session: Session,
) -> Result<HttpResponse, Error> {
    let session_data = update_session(session)?;
    use rustgym_schema::schema::google_problem::dsl::*;
    let conn = conn(pool)?;
    let all_rows: Vec<GoogleIndexRow> = google_problem
        .select((id, division, year, round, title))
        .load(&conn)
        .map_err(ErrorNotFound)?;
    let codejam_rows: Vec<GoogleIndexRow> = all_rows
        .iter()
        .filter(|r| r.division == 1)
        .cloned()
        .collect();
    let kickstart_rows: Vec<GoogleIndexRow> = all_rows
        .iter()
        .filter(|r| r.division == 2)
        .cloned()
        .collect();
    GoogleIndexContext::new(
        AppContext::new(data),
        session_data,
        req.path().to_string(),
        codejam_rows,
        kickstart_rows,
    )
    .render_wrapper()
}
