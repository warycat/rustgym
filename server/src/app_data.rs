use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;

#[derive(Clone)]
pub struct AppData {
    pub tag: RefCell<String>,
    pub title: RefCell<String>,
    pub turn_static_auth_secret: RefCell<String>,
    pub sessions: Arc<Mutex<HashMap<u128, i32>>>,
}

impl AppData {
    pub fn new(tag: String, title: String, turn_static_auth_secret: String) -> AppData {
        AppData {
            tag: RefCell::new(tag),
            title: RefCell::new(title),
            turn_static_auth_secret: RefCell::new(turn_static_auth_secret),
            sessions: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}
