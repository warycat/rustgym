use rustgym_msg::*;
use seed::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
use web_sys::MediaStream;

pub struct Model {
    pub client_info: Option<ClientInfo>,
    pub search_text: String,
    pub search_suggestions: Vec<String>,
    pub query_results: Vec<QueryResult>,
    pub web_socket: Rc<RefCell<WebSocket>>,
    pub web_socket_errors: Vec<WebSocketError>,
    pub media_stream: Option<MediaStream>,
    pub all_clients: Vec<ClientInfo>,
}
