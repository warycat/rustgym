use rustgym_msg::ClientInfo;
use rustgym_msg::Msg;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, new)]
pub struct Envelope {
    pub client_info: ClientInfo,
    pub msg: Msg,
}
