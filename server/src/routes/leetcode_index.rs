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
use rustgym_schema::*;

#[get("/leetcode")]
pub async fn leetcode_index(
    data: web::Data<AppData>,
    req: HttpRequest,
    pool: web::Data<SqlitePool>,
    session: Session,
) -> Result<HttpResponse, Error> {
    let session_data = update_session(session)?;
    use schema::leetcode_description::dsl::*;
    use schema::leetcode_question::dsl::*;
    let conn = conn(pool)?;
    let rows: Vec<LeetcodeIndexRow> = leetcode_question
        .select((schema::leetcode_question::dsl::id, title, level))
        .inner_join(leetcode_description)
        .order(schema::leetcode_question::dsl::id)
        .load(&conn)
        .map_err(ErrorNotFound)?;

    let mut rows_easy = vec![];
    let mut rows_medium = vec![];
    let mut rows_hard = vec![];
    for row in rows {
        match row.level {
            1 => {
                rows_easy.push(row);
            }
            2 => {
                rows_medium.push(row);
            }
            3 => {
                rows_hard.push(row);
            }
            _ => {
                panic!();
            }
        }
    }
    LeetcodeIndexContext::new(
        AppContext::new(data),
        session_data,
        req.path().to_string(),
        rows_easy,
        rows_medium,
        rows_hard,
    )
    .render_wrapper()
}
