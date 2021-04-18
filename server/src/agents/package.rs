use crate::agents::envelope::Envelope;
use crate::agents::websocket::SocketClient;
use actix::prelude::*;
use rustgym_msg::ClientInfo;
use rustgym_msg::Msg;

#[derive(Message, Clone, new)]
#[rtype(result = "()")]
pub struct Package {
    pub client_addr: Addr<SocketClient>,
    pub envelope: Envelope,
}

impl Package {
    pub fn from_message(
        client_addr: Addr<SocketClient>,
        client_info: ClientInfo,
        msg: Msg,
    ) -> Self {
        let envelope = Envelope::new(client_info, msg);
        Package {
            client_addr,
            envelope,
        }
    }
}
