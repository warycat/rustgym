use crate::agents::websocket::SocketClient;
use actix::prelude::*;
use actix_web::web::Bytes;
use rustgym_msg::ClientInfo;
use rustgym_msg::Msg;

#[derive(Message, Clone, new)]
#[rtype(result = "()")]
pub struct Chunk {
    pub client_addr: Addr<SocketClient>,
    pub client_info: ClientInfo,
    pub bytes: Bytes,
}
