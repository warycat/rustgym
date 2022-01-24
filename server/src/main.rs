#[macro_use]
extern crate diesel;
#[macro_use]
extern crate derive_new;

mod agents;
mod app_data;
mod context;
mod db;
mod files;
mod routes;
mod session_data;

use actix::prelude::*;
use actix_session::CookieSession;
use actix_web::middleware::Logger;
use actix_web::web;
use actix_web::App;
use actix_web::HttpServer;
use agents::registry::RegistryAgent;
use agents::search::SearchAgent;
use agents::uap::UapAgent;
use app_data::AppData;
use db::*;
use log::info;
use rustgym_consts::*;
use std::env;
use std::process::Command;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
    let pool: SqlitePool = db::init_pool(DATABASE_URL).expect("Failed to create pool");
    let tag = env::var("TAG").expect("TAG");
    let turn_static_auth_secret =
        env::var("TURN_STATIC_AUTH_SECRET").expect("TURN_STATIC_AUTH_SECRET");
    let title = "Rust Gym".to_string();
    info!("{} {}", title, tag);
    let app_data = AppData::new(tag, title, turn_static_auth_secret);
    let search_addr = SearchAgent::new(pool.clone()).start();
    let registry_addr = RegistryAgent::new(search_addr).start();
    let uap_addr = UapAgent::new().start();
    Command::new("mkdir")
        .arg(STREAM_DIR)
        .output()
        .expect("Create Stream Dir");
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(CookieSession::signed(&[0; 32]).secure(false))
            .data(app_data.clone())
            .data(pool.clone())
            .data(registry_addr.clone())
            .data(uap_addr.clone())
            .service(routes::home::home)
            .service(routes::find::find)
            .service(routes::nes_index::nes_index)
            .service(routes::nes_detail::nes_detail)
            .service(routes::leetcode_index::leetcode_index)
            .service(routes::adventofcode_index::adventofcode_index)
            .service(routes::google_index::google_index)
            .service(routes::leetcode_detail::leetcode_detail)
            .service(routes::adventofcode_detail::adventofcode_detail)
            .service(routes::google_detail::google_detail)
            .service(routes::robots::robots_txt)
            .service(routes::sitemap::sitemap_txt)
            .service(files::client_files)
            .service(files::static_files)
            .service(files::data_files)
            .service(files::favicon_file)
            .service(web::resource("/ws/").to(agents::websocket::ws_index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
