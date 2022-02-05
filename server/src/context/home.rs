use crate::context::*;

#[derive(Template, new)]
#[template(path = "home.j2")]
pub struct HomeContext {
    pub app: AppContext,
    pub session: SessionData,
    pub path: String,
}
