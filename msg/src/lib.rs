use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub enum Msg {
    Ping,
    Pong,
    ReqRegistorClient,
    ResRegistorClient {
        client_uuid: Uuid,
        session_uuid: Uuid,
        name: String,
    },
    ReqUnRegistorClient {
        client_uuid: Uuid,
        session_uuid: Uuid,
    },
}
