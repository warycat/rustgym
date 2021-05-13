use rustgym_msg::*;
use seed::prelude::*;
use uuid::Uuid;
use web_sys::{MediaStream, SourceBuffer};

pub enum Message {
    UrlChanged(subs::UrlChanged),
    SearchTextChanged(String),
    QueryText(String),
    KeyDown(web_sys::KeyboardEvent),
    WebSocketMsgOut(MsgOut),
    WebSocketMsgBin(MsgBin),
    WebSocketClosed(CloseEvent),
    WebSocketError(WebSocketError),
    MediaStreamReady(MediaStream),
    SourceBuffer(Uuid, SourceBuffer),
    AllClients(Vec<ClientInfo>),
    WebSocketOpened,
    WebSocketFailed,
}
