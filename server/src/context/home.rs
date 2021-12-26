use crate::context::*;

pub struct AppContext {
    pub title: String,
    pub tag: String,
}

#[derive(Template, new)]
#[template(path = "home.j2")]
pub struct HomeContext {
    pub app: AppContext,
    pub session: SessionData,
    pub path: String,
}

impl AppContext {
    pub fn new(data: Data<AppData>) -> Self {
        let title = data.title.borrow().to_string();
        let tag = data.tag.borrow().to_string();
        AppContext { title, tag }
    }
}
