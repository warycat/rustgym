use crate::agents::envelope::Envelope;
use crate::agents::search::SearchAgent;
use crate::agents::websocket::SocketClient;
use actix::prelude::*;
use log::{error, info};
use rustgym_msg::ClientInfo;
use rustgym_msg::*;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Clone)]
pub struct RegistryAgent {
    search_addr: Addr<SearchAgent>,
    all_clients: HashMap<Uuid, ClientInfo>,
    all_sockets: HashMap<Uuid, Addr<SocketClient>>,
}

impl RegistryAgent {
    pub fn new(search_addr: Addr<SearchAgent>) -> Self {
        let all_clients = HashMap::new();
        let all_sockets = HashMap::new();

        RegistryAgent {
            search_addr,
            all_sockets,
            all_clients,
        }
    }

    fn all_clients(&self) -> Vec<ClientInfo> {
        self.all_clients.values().cloned().collect()
    }

    fn update_all_clients(&self, msg_out: MsgOut) {
        for client_info in self.all_clients.values() {
            if let Some(client_addr) = self.all_sockets.get(&client_info.client_uuid) {
                let msg_out = msg_out.clone();
                let client_addr_clone = client_addr.clone();
                let envelope =
                    Envelope::from_msg_out(client_addr_clone, client_info.client_uuid, msg_out);
                client_addr.do_send(envelope);
            } else {
                error!("{} recipient not found", client_info.client_uuid);
            }
        }
    }

    fn update_client(&self, client_uuid: Uuid, msg_out: MsgOut) {
        if let Some(client_addr) = self.all_sockets.get(&client_uuid) {
            let envelope = Envelope::from_msg_out(client_addr.clone(), client_uuid, msg_out);
            client_addr.do_send(envelope);
        } else {
            error!("{} recipient not found", client_uuid);
        }
    }
}

impl Actor for RegistryAgent {
    type Context = Context<Self>;
}

impl Handler<Envelope> for RegistryAgent {
    type Result = ();

    fn handle(&mut self, envelope: Envelope, ctx: &mut Context<Self>) {
        let Envelope {
            client_addr,
            client_uuid,
            msg,
        } = envelope.clone();

        match msg {
            Msg::In(msg_in) => match msg_in {
                MsgIn::SearchText(_) => {
                    self.search_addr.do_send(envelope);
                }
                MsgIn::QueryText(_) => {
                    self.search_addr.do_send(envelope);
                }
                MsgIn::StreamStart { client_uuid } => {
                    let msg_out = MsgOut::StreamStart { client_uuid };
                    self.update_all_clients(msg_out);
                }
                MsgIn::Offer {
                    caller,
                    callee,
                    offer_sdp,
                } => {
                    let msg_out = MsgOut::Offer {
                        caller,
                        callee,
                        offer_sdp,
                    };
                    self.update_client(callee, msg_out);
                }
                MsgIn::Answer {
                    caller,
                    callee,
                    answer_sdp,
                } => {
                    let msg_out = MsgOut::Answer {
                        caller,
                        callee,
                        answer_sdp,
                    };
                    self.update_client(caller, msg_out);
                }
                MsgIn::IceCandidate {
                    local,
                    remote,
                    candidate,
                    sdp_mid,
                    sdp_m_line_index,
                } => {
                    let msg_out = MsgOut::IceCandidate {
                        local: remote,
                        remote: local,
                        candidate,
                        sdp_mid,
                        sdp_m_line_index,
                    };
                    self.update_client(remote, msg_out);
                }
                _ => {
                    error!("{:?}", msg_in);
                }
            },
            Msg::Out(msg_out) => match msg_out.clone() {
                MsgOut::RegistorClient(client_info) => {
                    self.all_sockets.entry(client_uuid).or_insert(client_addr);
                    self.all_clients.entry(client_uuid).or_insert(client_info);
                    self.update_all_clients(msg_out);
                }
                MsgOut::UnRegistorClient(_) => {
                    self.all_sockets.remove(&client_uuid);
                    self.all_clients.remove(&client_uuid);
                    self.update_all_clients(msg_out);
                }
                _ => {
                    error!("{:?}", msg_out);
                }
            },
        }
    }
}
