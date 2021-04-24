use crate::message::Message;
use crate::model::Model;
use crate::utils::*;
use rustgym_msg::*;
use seed::{prelude::*, *};
use std::cell::RefCell;
use std::rc::Rc;

fn decode_message(message: WebSocketMessage, msg_sender: Rc<dyn Fn(Option<Message>)>) {
    if message.contains_text() {
        let msg_out = message
            .json::<rustgym_msg::MsgOut>()
            .expect("Failed to decode WebSocket text message");
        msg_sender(Some(Message::WebSocketMsgOut(msg_out)));
    } else {
        spawn_local(async move {
            let bytes: Vec<u8> = message
                .bytes()
                .await
                .expect("WebsocketError on binary data");

            let msg_bin: MsgBin = bincode::deserialize(&bytes).unwrap();
            msg_sender(Some(Message::WebSocketMsgBin(msg_bin)));
        });
    }
}

fn web_socket(orders: &mut impl Orders<Message>) -> WebSocket {
    let msg_sender = orders.msg_sender();
    WebSocket::builder(wsurl().expect("url"), orders)
        .on_open(|| Message::WebSocketOpened)
        .on_message(|msg: WebSocketMessage| decode_message(msg, msg_sender))
        .on_close(Message::WebSocketClosed)
        .on_error(|| Message::WebSocketFailed)
        .build_and_open()
        .expect("web_socket")
}

pub fn init(_url: seed::Url, orders: &mut impl Orders<Message>) -> Model {
    orders.subscribe(Message::UrlChanged);

    let client_info = None;
    let web_socket = Rc::new(RefCell::new(web_socket(orders)));
    let search_text = "".to_string();
    let web_socket_errors = vec![];
    let search_suggestions = vec![];
    let query_results = vec![];
    let media_stream = None;
    let all_clients = vec![];
    Model {
        client_info,
        search_text,
        search_suggestions,
        query_results,
        web_socket,
        web_socket_errors,
        media_stream,
        all_clients,
    }
}
