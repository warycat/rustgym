use crate::message::Message;
use crate::utils::*;
use rustgym_msg::*;
use seed::{prelude::*, *};
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

pub fn web_socket_builder(orders: &mut impl Orders<Message>) -> WebSocket {
    let msg_sender = orders.msg_sender();
    WebSocket::builder(wsurl().expect("url"), orders)
        .on_open(|| Message::WebSocketOpened)
        .on_message(|msg: WebSocketMessage| decode_message(msg, msg_sender))
        .on_close(Message::WebSocketClosed)
        .on_error(|| Message::WebSocketFailed)
        .build_and_open()
        .expect("web_socket")
}
