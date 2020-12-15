use super::context::LeetcodeIndexContext;
use super::context::LeetcodeIndexRow;
use super::db;
use actix_web::error::ErrorInternalServerError;
use actix_web::error::ErrorNotFound;
use actix_web::get;
use actix_web::web;
use actix_web::Error;
use actix_web::HttpResponse;
use diesel::prelude::*;

#[get("/leetcode/")]
pub async fn leetcode_index(pool: web::Data<db::SqlitePool>) -> Result<HttpResponse, Error> {
    use rustgym_schema::schema::leetcode_description::dsl::*;
    use rustgym_schema::schema::leetcode_question::dsl::*;
    let conn = pool.get().map_err(ErrorInternalServerError)?;
    let rows: Vec<LeetcodeIndexRow> = leetcode_question
        .select((frontend_id, title))
        .inner_join(leetcode_description)
        .load(&conn)
        .map_err(ErrorNotFound)?;
    let template = LeetcodeIndexContext { rows };
    let body = template.render_wrapper()?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}
