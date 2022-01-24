mod adventofcode;
mod app;
mod find;
mod google;
mod home;
mod leetcode;
mod nes;
mod sitemap;

pub use adventofcode::*;
pub use app::*;
pub use find::*;
pub use google::*;
pub use home::*;
pub use leetcode::*;
pub use nes::*;
pub use sitemap::*;

use crate::session_data::SessionData;
use crate::AppData;
use actix_web::error::ErrorInternalServerError;
use actix_web::web::Data;
use actix_web::Error;
use actix_web::HttpResponse;
use askama::Template;
use rustgym_schema::*;

macro_rules! impl_render_wrapper {
    ($type: ty) => {
        impl $type {
            pub fn render_wrapper(&self) -> Result<HttpResponse, Error> {
                let body = self.render().map_err(ErrorInternalServerError)?;
                Ok(HttpResponse::Ok().content_type("text/html").body(body))
            }
        }
    };
}

macro_rules! impl_txt_render_wrapper {
    ($type: ty) => {
        impl $type {
            pub fn render_wrapper(&self) -> Result<HttpResponse, Error> {
                let body = self.render().map_err(ErrorInternalServerError)?;
                Ok(HttpResponse::Ok().content_type("text/txt").body(body))
            }
        }
    };
}

impl_render_wrapper!(HomeContext);
impl_render_wrapper!(LeetcodeIndexContext);
impl_render_wrapper!(AdventOfCodeIndexContext);
impl_render_wrapper!(LeetcodeDetailContext);
impl_render_wrapper!(AdventOfCodeDetailContext);
impl_render_wrapper!(GoogleIndexContext);
impl_render_wrapper!(GoogleDetailContext);
impl_render_wrapper!(NesIndexContext);
impl_render_wrapper!(NesDetailContext);
impl_render_wrapper!(FindContext);

impl_txt_render_wrapper!(SitemapContext);
impl_txt_render_wrapper!(RobotsContext);
