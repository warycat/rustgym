use crate::agents::websocket::SocketClient;
use actix::prelude::*;
use rustgym_msg::*;
use std::fmt;
use uuid::Uuid;

#[derive(Message, Debug, Clone, new)]
#[rtype(result = "()")]
pub struct Envelope {
    pub client_addr: Addr<SocketClient>,
    pub client_uuid: Uuid,
    pub msg: Msg,
}

impl Envelope {
    pub fn from_msg_in(client_addr: Addr<SocketClient>, client_uuid: Uuid, msg_in: MsgIn) -> Self {
        let msg = Msg::In(msg_in);
        Envelope {
            client_addr,
            client_uuid,
            msg,
        }
    }
    pub fn from_msg_out(
        client_addr: Addr<SocketClient>,
        client_uuid: Uuid,
        msg_out: MsgOut,
    ) -> Self {
        let msg = Msg::Out(msg_out);
        Envelope {
            client_addr,
            client_uuid,
            msg,
        }
    }
}

impl fmt::Debug for SocketClient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<SocketClient>")
    }
}
