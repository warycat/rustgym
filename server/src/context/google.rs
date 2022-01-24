use crate::context::*;

#[derive(Queryable, Clone)]
pub struct GoogleIndexRow {
    pub id: i32,
    pub division: i32,
    pub year: i32,
    pub round: i32,
    pub title: String,
}

impl GoogleIndexRow {
    pub fn href(&self) -> String {
        format!("/google/{}", self.id)
    }
}

#[derive(Template, new)]
#[template(path = "google-index.j2")]
pub struct GoogleIndexContext {
    pub app: AppContext,
    pub session: SessionData,
    pub path: String,
    pub rows_codejam: Vec<GoogleIndexRow>,
    pub rows_kickstart: Vec<GoogleIndexRow>,
}

#[derive(Template, new)]
#[template(path = "google-detail.j2")]
pub struct GoogleDetailContext {
    pub app: AppContext,
    pub session: SessionData,
    pub path: String,
    pub item: GoogleProblem,
}
