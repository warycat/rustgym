#[macro_use]
extern crate derive_new;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Msg {
    In(MsgIn),
    Out(MsgOut),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MsgIn {
    Ping,
    Pong,
    SearchText(String),
    QueryText(String),
    StreamStart {
        client_uuid: Uuid,
    },
    Offer {
        caller: Uuid,
        callee: Uuid,
        offer_sdp: String,
    },
    Answer {
        caller: Uuid,
        callee: Uuid,
        answer_sdp: String,
    },
    IceCandidate {
        local: Uuid,
        remote: Uuid,
        candidate: String,
        sdp_mid: String,
        sdp_m_line_index: u16,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MsgOut {
    Ping,
    Pong,
    SearchSuggestions(Vec<String>),
    QueryResults(Vec<QueryResult>),
    RegistorClient(ClientInfo),
    UnRegistorClient(ClientInfo),
    StreamStart {
        client_uuid: Uuid,
    },
    Offer {
        caller: Uuid,
        callee: Uuid,
        offer_sdp: String,
    },
    Answer {
        caller: Uuid,
        callee: Uuid,
        answer_sdp: String,
    },
    IceCandidate {
        local: Uuid,
        remote: Uuid,
        candidate: String,
        sdp_mid: String,
        sdp_m_line_index: u16,
    },
    SessionClients(Vec<ClientInfo>),
    AllClients(Vec<ClientInfo>),
}

#[derive(Serialize, Deserialize, Debug, Clone, Hash, Eq, PartialEq, new)]
pub struct UserAgent {
    pub family: String,
    pub major: Option<String>,
    pub minor: Option<String>,
    pub patch: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Hash, Eq, PartialEq, new)]
pub struct ClientInfo {
    pub session_uuid: Uuid,
    pub client_uuid: Uuid,
    pub name: String,
    pub user_agent: Option<UserAgent>,
    pub streaming: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
pub struct QueryResult {
    pub id: String,
    pub title: String,
    pub href: String,
    pub from: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
pub struct MsgBin {
    pub uuid: Uuid,
    pub bytes: Vec<u8>,
}
