use crate::agents::envelope::Envelope;
use crate::agents::websocket::SocketClient;
use actix::prelude::*;

#[derive(Message)]
#[rtype(result = "()")]
pub struct Package {
    pub client_addr: Addr<SocketClient>,
    pub envelope: Envelope,
}
