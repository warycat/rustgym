#[macro_use]
extern crate diesel;
#[macro_use]
extern crate derive_new;

mod adventofcode_detail;
mod adventofcode_index;
mod app_data;
mod client;
mod context;
mod db;
mod home;
mod leetcode_detail;
mod leetcode_index;
mod robots;
mod sitemap;

use actix_session::CookieSession;
use actix_web::middleware::Logger;
use actix_web::App;
use actix_web::HttpServer;
use app_data::AppData;
use rustgym_consts::*;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    env_logger::init();
    let pool = db::init_pool(DATABASE_URL).expect("Failed to create pool");
    let tag = env::var("TAG").unwrap_or_default();
    let title = "RUST GYM".to_string();
    println!("RUST GYM Server {}", tag);
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(CookieSession::signed(&[0; 32]).secure(false))
            .data(AppData::new(tag.clone(), title.clone()))
            .data(pool.clone())
            .service(home::home)
            .service(leetcode_index::leetcode_index)
            .service(adventofcode_index::adventofcode_index)
            .service(leetcode_detail::leetcode_detail)
            .service(adventofcode_detail::adventofcode_detail)
            .service(robots::robots_txt)
            .service(sitemap::sitemap_txt)
            .service(client::client_files)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
