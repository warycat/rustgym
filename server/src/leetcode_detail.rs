use super::app_data::AppData;
use super::context::*;
use super::db::*;
use actix_web::error::ErrorNotFound;
use actix_web::get;
use actix_web::web;
use actix_web::Error;
use actix_web::HttpResponse;
use diesel::prelude::*;
use rustgym_schema::LeetcodeDescription;
use rustgym_schema::LeetcodeSolution;

#[get("/leetcode/{id}")]
async fn leetcode_detail(
    data: web::Data<AppData>,
    web::Path(id_): web::Path<i32>,
    pool: web::Data<SqlitePool>,
) -> Result<HttpResponse, Error> {
    use rustgym_schema::schema::leetcode_description::dsl::*;
    use rustgym_schema::schema::leetcode_question::dsl::*;
    use rustgym_schema::schema::leetcode_solution::dsl::*;
    let conn = conn(pool)?;
    let description = leetcode_description
        .find(id_)
        .first(&conn)
        .map_err(ErrorNotFound)
        // remove after most descriptions are added.
        .unwrap_or_else(|_| LeetcodeDescription::new(id_, "".to_string(), "".to_string()));
    let question = leetcode_question
        .filter(rustgym_schema::schema::leetcode_question::dsl::id.eq(id_))
        .first(&conn)
        .map_err(ErrorNotFound)?;
    let solutions: Vec<LeetcodeSolution> = leetcode_solution
        .filter(question_id.eq(id_))
        .load(&conn)
        .map_err(ErrorNotFound)?;
    LeetcodeDetailContext::new(AppContext::new(data), question, description, solutions)
        .render_wrapper()
}
