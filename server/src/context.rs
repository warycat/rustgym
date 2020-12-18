use super::AppData;
use actix_web::error::ErrorInternalServerError;
use actix_web::web::Data;
use actix_web::Error;
use actix_web::HttpResponse;
use askama::Template;
use rustgym_schema::leetcode_description::LeetcodeDescription;
use rustgym_schema::leetcode_question::LeetcodeQuestion;
use rustgym_schema::leetcode_solution::LeetcodeSolution;

pub struct AppContext {
    pub title: String,
    pub tag: String,
}

#[derive(Template, new)]
#[template(path = "home.j2")]
pub struct HomeContext {
    pub app: AppContext,
}

impl AppContext {
    pub fn new(data: Data<AppData>) -> Self {
        let title = data.title.borrow().to_string();
        let tag = data.tag.borrow().to_string();
        AppContext { title, tag }
    }
}

#[derive(Queryable)]
pub struct LeetcodeIndexRow {
    pub qid: i32,
    pub title: String,
    pub level: i32,
}

#[derive(Template, new)]
#[template(path = "leetcode-index.j2")]
pub struct LeetcodeIndexContext {
    pub app: AppContext,
    pub rows: Vec<LeetcodeIndexRow>,
}

#[derive(Template, new)]
#[template(path = "leetcode-detail.j2")]
pub struct LeetcodeDetailContext {
    pub app: AppContext,
    pub question: LeetcodeQuestion,
    pub description: LeetcodeDescription,
    pub solutions: Vec<LeetcodeSolution>,
}

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

impl_render_wrapper!(HomeContext);
impl_render_wrapper!(LeetcodeIndexContext);
impl_render_wrapper!(LeetcodeDetailContext);
