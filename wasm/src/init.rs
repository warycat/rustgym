use crate::message::Message;
use crate::model::Model;
use crate::utils::*;
use seed::{prelude::*, *};
use std::cell::RefCell;
use std::rc::Rc;

fn web_socket(orders: &mut impl Orders<Message>) -> WebSocket {
    WebSocket::builder(wsurl().expect("url"), orders)
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
        .expect("web_socket")
}

pub fn init(url: seed::Url, orders: &mut impl Orders<Message>) -> Model {
    orders.subscribe(Message::UrlChanged);

    let web_socket = Rc::new(RefCell::new(web_socket(orders)));
    let search_text = "".to_string();
    let web_socket_errors = vec![];
    let search_suggestions = vec![];
    let query_results = vec![];
    let media_stream = None;
    Model {
        search_text,
        search_suggestions,
        query_results,
        web_socket,
        web_socket_errors,
        media_stream,
    }
}
