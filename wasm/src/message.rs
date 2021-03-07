use rustgym_msg::Msg;
use seed::prelude::*;

pub enum Message {
    UrlChanged(subs::UrlChanged),
    SearchTextChanged(String),
    QueryText(String),
    KeyDown(web_sys::KeyboardEvent),
    WebSocketMsg(Msg),
    WebSocketClosed(CloseEvent),
    WebSocketError(WebSocketError),
    WebSocketOpened,
    WebSocketFailed,
}
