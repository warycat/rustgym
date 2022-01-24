use crate::context::*;

pub struct AppContext {
    pub title: String,
    pub tag: String,
}

impl AppContext {
    pub fn new(data: Data<AppData>) -> Self {
        let title = data.title.borrow().to_string();
        let tag = data.tag.borrow().to_string();
        AppContext { title, tag }
    }
}
