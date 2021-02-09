use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;

#[derive(Clone)]
pub struct AppData {
    pub tag: RefCell<String>,
    pub title: RefCell<String>,
    pub sessions: Arc<Mutex<HashMap<u128, i32>>>,
}

impl AppData {
    pub fn new(tag: String, title: String) -> AppData {
        AppData {
            tag: RefCell::new(tag),
            title: RefCell::new(title),
            sessions: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}
