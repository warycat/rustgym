use super::app_data::AppData;
use super::context::*;
use super::db::*;
use actix_web::error::ErrorNotFound;
use actix_web::get;
use actix_web::web;
use actix_web::Error;
use actix_web::HttpResponse;
use diesel::prelude::*;

#[get("/adventofcode/{id}")]
async fn adventofcode_detail(
    data: web::Data<AppData>,
    web::Path(id_): web::Path<i32>,
    pool: web::Data<SqlitePool>,
) -> Result<HttpResponse, Error> {
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
    AdventOfCodeDetailContext::new(AppContext::new(data), description, solution).render_wrapper()
}
