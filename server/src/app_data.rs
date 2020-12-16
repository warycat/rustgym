use std::cell::RefCell;

#[derive(Clone)]
pub struct AppData {
    pub tag: RefCell<String>,
    pub title: RefCell<String>,
}

impl AppData {
    pub fn new(tag: String, title: String) -> AppData {
        AppData {
            tag: RefCell::new(tag),
            title: RefCell::new(title),
        }
    }
}
