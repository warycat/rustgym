use rustgym_msg::QueryResult;
use seed::prelude::*;

pub struct Model {
    pub search_text: String,
    pub search_suggestions: Vec<String>,
    pub query_results: Vec<QueryResult>,
    pub web_socket: WebSocket,
    pub web_socket_errors: Vec<WebSocketError>,
}
