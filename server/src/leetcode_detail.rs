use super::context::LeetcodeDetailContext;
use super::db::*;
use actix_web::error::ErrorNotFound;
use actix_web::get;
use actix_web::web;
use actix_web::Error;
use actix_web::HttpResponse;
use diesel::prelude::*;

#[get("/leetcode/{leetcode_id}")]
async fn leetcode_detail(
    web::Path(leetcode_id): web::Path<i32>,
    pool: web::Data<SqlitePool>,
) -> Result<HttpResponse, Error> {
    use rustgym_schema::schema::leetcode_description::dsl::*;
    use rustgym_schema::schema::leetcode_question::dsl::*;
    let conn = conn(pool)?;
    let description = leetcode_description
        .find(leetcode_id)
        .first(&conn)
        .map_err(ErrorNotFound)?;
    let question = leetcode_question
        .filter(frontend_id.eq(leetcode_id))
        .first(&conn)
        .map_err(ErrorNotFound)?;
    LeetcodeDetailContext::new(question, description).render_wrapper()
}
