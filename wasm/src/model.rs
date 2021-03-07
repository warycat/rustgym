use seed::prelude::*;

pub struct Model {
    pub search_text: String,
    pub suggestions: Vec<String>,
    pub results: Vec<(String, String)>,
    pub web_socket: WebSocket,
    pub web_socket_errors: Vec<WebSocketError>,
}
