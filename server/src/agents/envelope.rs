use uuid::Uuid;

use rustgym_msg::Msg;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Envelope {
    pub client_uuid: Uuid,
    pub session_uuid: Uuid,
    pub msg: Msg,
}
