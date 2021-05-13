use rustgym_msg::*;
use seed::prelude::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use uuid::Uuid;
use web_sys::{MediaRecorder, MediaStream, SourceBuffer};

pub struct Model {
    pub client_info: Option<ClientInfo>,
    pub search_text: String,
    pub search_suggestions: Vec<String>,
    pub query_results: Vec<QueryResult>,
    pub web_socket: Rc<RefCell<WebSocket>>,
    pub web_socket_errors: Vec<WebSocketError>,
    pub all_clients: Vec<ClientInfo>,
    pub all_source_urls: HashMap<Uuid, String>,
    pub all_source_buffers: HashMap<Uuid, SourceBuffer>,
    pub media_stream: Option<MediaStream>,
    pub media_recorder: Option<MediaRecorder>,
}

impl Model {
    pub fn new(websocket: seed::prelude::WebSocket) -> Self {
        let client_info = None;
        let web_socket = Rc::new(RefCell::new(websocket));
        let search_text = "".to_string();
        let web_socket_errors = vec![];
        let search_suggestions = vec![];
        let query_results = vec![];
        let media_stream = None;
        let all_clients = vec![];
        let all_source_buffers = HashMap::new();
        let all_source_urls = HashMap::new();
        let media_recorder = None;
        Model {
            client_info,
            search_text,
            search_suggestions,
            query_results,
            web_socket,
            web_socket_errors,
            all_clients,
            all_source_buffers,
            all_source_urls,
            media_stream,
            media_recorder,
        }
    }

    pub fn reconnect(&mut self, websocket: seed::prelude::WebSocket) {
        self.web_socket = Rc::new(RefCell::new(websocket));
    }
}
