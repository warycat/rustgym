use crate::agents::websocket::SocketClient;
use actix::prelude::*;
use rustgym_msg::ClientInfo;
use rustgym_msg::*;
use std::fmt;

#[derive(Message, Debug, Clone, new)]
#[rtype(result = "()")]
pub struct Envelope {
    pub client_addr: Addr<SocketClient>,
    pub client_info: ClientInfo,
    pub msg: Msg,
}

impl Envelope {
    pub fn from_msg_in(
        client_addr: Addr<SocketClient>,
        client_info: ClientInfo,
        msg_in: MsgIn,
    ) -> Self {
        let msg = Msg::In(msg_in);
        Envelope {
            client_addr,
            client_info,
            msg,
        }
    }
    pub fn from_msg_out(
        client_addr: Addr<SocketClient>,
        client_info: ClientInfo,
        msg_out: MsgOut,
    ) -> Self {
        let msg = Msg::Out(msg_out);
        Envelope {
            client_addr,
            client_info,
            msg,
        }
    }
}

impl fmt::Debug for SocketClient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<SocketClient>")
    }
}
