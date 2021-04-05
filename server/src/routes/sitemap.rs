use crate::context::*;
use crate::db::*;
use actix_web::error::ErrorNotFound;
use actix_web::get;
use actix_web::web;
use actix_web::Error;
use actix_web::HttpResponse;
use diesel::prelude::*;

#[get("/sitemap.txt")]
pub async fn sitemap_txt(pool: web::Data<SqlitePool>) -> Result<HttpResponse, Error> {
    use rustgym_schema::schema::adventofcode_description::dsl::*;
    use rustgym_schema::schema::google_problem::dsl::*;
    use rustgym_schema::schema::leetcode_description::dsl::*;
    let conn = conn(pool)?;
    let leetcode_rows = leetcode_description.load(&conn).map_err(ErrorNotFound)?;
    let adventofcode_rows = adventofcode_description
        .load(&conn)
        .map_err(ErrorNotFound)?;
    let google_rows = google_problem.load(&conn).map_err(ErrorNotFound)?;
    SitemapContext::new(leetcode_rows, adventofcode_rows, google_rows).render_wrapper()
}
