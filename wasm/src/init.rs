use crate::message::Message;
use crate::model::Model;
use seed::prelude::*;
use web_sys::{Location, Window};

fn wsurl() -> Result<String, JsValue> {
    let window: Window = web_sys::window().expect("no global `window` exists");
    let location: Location = window.location();
    let protocol: String = location.protocol()?;
    let host: String = location.host()?;
    let ws_protocol = if protocol == "https:" {
        "wss://"
    } else {
        "ws://"
    };
    Ok(format!("{}{}/ws/", ws_protocol, host))
}

pub fn init(url: Url, orders: &mut impl Orders<Message>) -> Model {
    orders.subscribe(Message::UrlChanged);
    let web_socket = WebSocket::builder(wsurl().expect("url"), orders)
        .on_open(|| Message::WebSocketOpened)
        .on_message(
            |msg: WebSocketMessage| match msg.json::<rustgym_msg::Msg>() {
                Ok(msg) => Message::WebSocketMsg(msg),
                Err(err) => Message::WebSocketError(err),
            },
        )
        .on_close(Message::WebSocketClosed)
        .on_error(|| Message::WebSocketFailed)
        .build_and_open()
        .expect("web_socket");
    let mut search_text = "".to_string();
    if let Some(v) = url.search().get("text") {
        if let Some(first) = v.first() {
            search_text = first.to_string();
        }
    }
    let web_socket_errors = vec![];
    let search_suggestions = vec![];
    let query_results = vec![];
    Model {
        search_text,
        search_suggestions,
        query_results,
        web_socket,
        web_socket_errors,
    }
}
