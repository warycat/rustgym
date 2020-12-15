mod db;

use actix_web::*;
use askama::Template;
use diesel::prelude::*;
use rustgym_consts::*;
use rustgym_schema::leetcode_question::LeetcodeQuestion; // bring trait in scope

#[derive(Template)]
#[template(path = "index.j2")]
struct IndexTemplate {
    title: String,
}

#[derive(Template)]
#[template(path = "leetcode.j2")]
struct LeetcodeTemplate {
    questions: Vec<LeetcodeQuestion>,
}

#[get("/leetcode")]
async fn greet(pool: web::Data<db::SqlitePool>) -> Result<HttpResponse, Error> {
    use rustgym_schema::schema::leetcode_question::dsl::*;
    let conn = pool.get().unwrap();
    let questions: Vec<LeetcodeQuestion> =
        leetcode_question.load::<LeetcodeQuestion>(&conn).unwrap();
    let leetcode_template = LeetcodeTemplate { questions };
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(leetcode_template.render().unwrap()))
}

#[get("/")]
async fn index() -> Result<HttpResponse, Error> {
    let index_template = IndexTemplate {
        title: "RustGym".to_string(),
    };
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(index_template.render().unwrap()))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = db::init_pool(DATABASE_URL).expect("Failed to create pool");

    println!("rustgym server v0.0.1");
    HttpServer::new(move || App::new().data(pool.clone()).service(index).service(greet))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
