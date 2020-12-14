mod db;

use actix_web::{web, App, HttpResponse, HttpServer};
use diesel::prelude::*;
use rustgym_consts::*;
use rustgym_schema::leetcode_question::LeetcodeQuestion;

async fn greet(pool: web::Data<db::SqlitePool>) -> HttpResponse {
    use rustgym_schema::schema::leetcode_question::dsl::*;
    let conn = pool.get().unwrap();
    let res: Vec<LeetcodeQuestion> = leetcode_question.load::<LeetcodeQuestion>(&conn).unwrap();
    HttpResponse::Ok().json(res)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = db::init_pool(DATABASE_URL).expect("Failed to create pool");

    println!("rustgym server v0.0.1");
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
