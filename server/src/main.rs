#[macro_use]
extern crate diesel;

mod db;

use actix_web::error::ErrorInternalServerError;
use actix_web::error::ErrorNotFound;
use actix_web::get;
use actix_web::web;
use actix_web::App;
use actix_web::Error;
use actix_web::HttpResponse;
use actix_web::HttpServer;

use askama::Template;
use diesel::prelude::*;
use rustgym_consts::*;
use rustgym_schema::leetcode_description::LeetcodeDescription;
use rustgym_schema::leetcode_question::LeetcodeQuestion; // bring trait in scope // bring trait in scope

#[derive(Template)]
#[template(path = "index.j2")]
struct IndexTemplate {
    title: String,
}

#[derive(Template)]
#[template(path = "leetcode-index.j2")]
struct LeetcodeIndexTemplate {
    rows: Vec<LeetcodeIndexRow>,
}

#[derive(Template)]
#[template(path = "leetcode-detail.j2")]
struct LeetcodeDetailTemplate {
    question: LeetcodeQuestion,
    description: LeetcodeDescription,
}

#[derive(Queryable)]
struct LeetcodeIndexRow {
    frontend_id: i32,
    title: String,
}

#[get("/leetcode/")]
async fn leetcode_index(pool: web::Data<db::SqlitePool>) -> Result<HttpResponse, Error> {
    use rustgym_schema::schema::leetcode_description::dsl::*;
    use rustgym_schema::schema::leetcode_question::dsl::*;
    let conn = pool.get().map_err(ErrorInternalServerError)?;
    let rows: Vec<LeetcodeIndexRow> = leetcode_question
        .select((frontend_id, title))
        .inner_join(leetcode_description)
        .load(&conn)
        .map_err(ErrorNotFound)?;
    let template = LeetcodeIndexTemplate { rows };
    let body = template.render().map_err(ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[get("/leetcode/{leetcode_id}")]
async fn leetcode_get(
    web::Path(leetcode_id): web::Path<i32>,
    pool: web::Data<db::SqlitePool>,
) -> Result<HttpResponse, Error> {
    use rustgym_schema::schema::leetcode_description::dsl::*;
    use rustgym_schema::schema::leetcode_question::dsl::*;
    let conn = pool.get().map_err(ErrorInternalServerError)?;
    let description = leetcode_description
        .find(leetcode_id)
        .first(&conn)
        .map_err(ErrorNotFound)?;
    let question = leetcode_question
        .filter(frontend_id.eq(leetcode_id))
        .first(&conn)
        .map_err(ErrorNotFound)?;
    let template = LeetcodeDetailTemplate {
        question,
        description,
    };
    let body = template.render().map_err(ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[get("/")]
async fn index() -> Result<HttpResponse, Error> {
    let template = IndexTemplate {
        title: "RustGym".to_string(),
    };
    let body = template.render().map_err(ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = db::init_pool(DATABASE_URL).expect("Failed to create pool");

    println!("rustgym server v0.0.1");
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .service(index)
            .service(leetcode_index)
            .service(leetcode_get)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
