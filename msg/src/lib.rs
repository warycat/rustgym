#[macro_use]
extern crate derive_new;

use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Msg {
    Ping,
    Pong,
    RegistorClient(ClientInfo),
    UnRegistorClient(ClientInfo),
    SessionClients(HashSet<ClientInfo>),
    SearchText(String),
    SearchSuggestions(Vec<String>),
    QueryText(String),
    QueryResults(Vec<QueryResult>),
}

#[derive(Serialize, Deserialize, Debug, Clone, Hash, Eq, PartialEq, new)]
pub struct ClientInfo {
    pub session_uuid: Uuid,
    pub client_uuid: Uuid,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
pub struct QueryResult {
    pub id: String,
    pub title: String,
    pub href: String,
    pub from: String,
}
