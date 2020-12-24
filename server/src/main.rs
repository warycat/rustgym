#[macro_use]
extern crate diesel;
#[macro_use]
extern crate derive_new;

mod adventofcode_detail;
mod adventofcode_index;
mod app_data;
mod context;
mod db;
mod home;
mod leetcode_detail;
mod leetcode_index;
mod sitemap;

use actix_web::App;
use actix_web::HttpServer;
use app_data::AppData;
use rustgym_consts::*;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = db::init_pool(DATABASE_URL).expect("Failed to create pool");
    let tag = env::var("TAG").unwrap_or_default();
    let title = "RUST GYM".to_string();
    println!("rustgym server {}", tag);
    HttpServer::new(move || {
        App::new()
            .data(AppData::new(tag.clone(), title.clone()))
            .data(pool.clone())
            .service(home::home)
            .service(leetcode_index::leetcode_index)
            .service(adventofcode_index::adventofcode_index)
            .service(leetcode_detail::leetcode_detail)
            .service(adventofcode_detail::adventofcode_detail)
            .service(sitemap::sitemap_txt)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
