use super::context::LeetcodeIndexContext;
use super::context::LeetcodeIndexRow;
use super::db::*;
use actix_web::error::ErrorNotFound;
use actix_web::get;
use actix_web::web;
use actix_web::Error;
use actix_web::HttpResponse;
use diesel::prelude::*;

#[get("/leetcode/")]
pub async fn leetcode_index(pool: web::Data<SqlitePool>) -> Result<HttpResponse, Error> {
    use rustgym_schema::schema::leetcode_description::dsl::*;
    use rustgym_schema::schema::leetcode_question::dsl::*;
    let conn = conn(pool)?;
    let rows: Vec<LeetcodeIndexRow> = leetcode_question
        .select((qid, title))
        .inner_join(leetcode_description)
        .load(&conn)
        .map_err(ErrorNotFound)?;
    LeetcodeIndexContext::new(rows).render_wrapper()
}
