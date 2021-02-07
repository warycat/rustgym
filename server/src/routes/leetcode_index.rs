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

#[get("/leetcode/")]
pub async fn leetcode_index(
    data: web::Data<AppData>,
    req: HttpRequest,
    pool: web::Data<SqlitePool>,
    session: Session,
) -> Result<HttpResponse, Error> {
    let session_data = update_session(session)?;
    use rustgym_schema::schema::leetcode_description::dsl::*;
    use rustgym_schema::schema::leetcode_question::dsl::*;
    let conn = conn(pool)?;
    let rows: Vec<LeetcodeIndexRow> = leetcode_question
        .select((
            rustgym_schema::schema::leetcode_question::dsl::id,
            title,
            level,
        ))
        .inner_join(leetcode_description)
        .order(rustgym_schema::schema::leetcode_question::dsl::id)
        .load(&conn)
        .map_err(ErrorNotFound)?;
    LeetcodeIndexContext::new(
        AppContext::new(data),
        session_data,
        req.path().to_string(),
        rows,
    )
    .render_wrapper()
}
