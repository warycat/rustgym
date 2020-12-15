use std::cell::RefCell;

#[derive(Clone)]
pub struct AppData {
    pub tag: RefCell<String>,
}

impl AppData {
    pub fn new(tag: String) -> AppData {
        AppData {
            tag: RefCell::new(tag),
        }
    }
}
