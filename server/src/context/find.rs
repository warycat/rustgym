use crate::context::*;

#[derive(Template, new)]
#[template(path = "find.j2")]
pub struct FindContext {
    pub app: AppContext,
    pub session: SessionData,
    pub path: String,
}
