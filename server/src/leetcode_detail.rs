use super::context::LeetcodeDetailContext;
use super::db::*;
use actix_web::error::ErrorNotFound;
use actix_web::get;
use actix_web::web;
use actix_web::Error;
use actix_web::HttpResponse;
use diesel::prelude::*;
use rustgym_schema::LeetcodeSolution;

#[get("/leetcode/{id}")]
async fn leetcode_detail(
    web::Path(id): web::Path<i32>,
    pool: web::Data<SqlitePool>,
) -> Result<HttpResponse, Error> {
    use rustgym_schema::schema::leetcode_description::dsl::*;
    use rustgym_schema::schema::leetcode_question::dsl::*;
    use rustgym_schema::schema::leetcode_solution::dsl::*;

    let conn = conn(pool)?;
    let description = leetcode_description
        .find(id)
        .first(&conn)
        .map_err(ErrorNotFound)?;
    let question = leetcode_question
        .filter(qid.eq(id))
        .first(&conn)
        .map_err(ErrorNotFound)?;
    let solutions: Vec<LeetcodeSolution> = leetcode_solution
        .filter(question_id.eq(id))
        .load(&conn)
        .map_err(ErrorNotFound)?;
    LeetcodeDetailContext::new(question, description, solutions).render_wrapper()
}
