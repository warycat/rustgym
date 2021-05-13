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
    StreamStart(String),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MsgOut {
    Ping,
    Pong,
    RegistorClient(ClientInfo),
    UnRegistorClient(ClientInfo),
    SessionClients(Vec<ClientInfo>),
    AllClients(Vec<ClientInfo>),
    SearchSuggestions(Vec<String>),
    QueryResults(Vec<QueryResult>),
}

#[derive(Serialize, Deserialize, Debug, Clone, Hash, Eq, PartialEq, new)]
pub struct ClientInfo {
    pub session_uuid: Uuid,
    pub client_uuid: Uuid,
    pub name: String,
    pub chrome: bool,
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
