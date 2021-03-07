use crate::agents::envelope::Envelope;
use crate::agents::package::Package;
use crate::agents::search::SearchAgent;
use crate::agents::websocket::SocketClient;
use actix::prelude::*;
use log::error;
use rustgym_msg::Msg;
use std::collections::HashMap;
use std::collections::HashSet;
use uuid::Uuid;

#[derive(Clone)]
pub struct RegistryAgent {
    search_addr: Addr<SearchAgent>,
    all_session_clients: HashMap<Uuid, HashSet<Uuid>>,
    all_clients: HashMap<Uuid, Addr<SocketClient>>,
}

impl RegistryAgent {
    pub fn new(search_addr: Addr<SearchAgent>) -> Self {
        let all_session_clients = HashMap::new();
        let all_clients = HashMap::new();

        RegistryAgent {
            search_addr,
            all_session_clients,
            all_clients,
        }
    }

    fn update_session_clients(&self, session_uuid: Uuid, msg: Msg) {
        if let Some(session_clients) = self.all_session_clients.get(&session_uuid) {
            for &client_uuid in session_clients {
                if let Some(client_addr) = self.all_clients.get(&client_uuid) {
                    let msg = msg.clone();
                    let client_addr_clone = client_addr.clone();
                    let envelope = Envelope {
                        session_uuid,
                        client_uuid,
                        msg,
                    };
                    let package = Package {
                        client_addr: client_addr_clone,
                        envelope,
                    };
                    client_addr.do_send(package);
                } else {
                    error!("{} recipient not found", client_uuid)
                }
            }
        } else {
            error!("{} not found", session_uuid);
        }
    }
}

impl Actor for RegistryAgent {
    type Context = Context<Self>;
}

impl Handler<Package> for RegistryAgent {
    type Result = ();

    fn handle(&mut self, package: Package, _ctx: &mut Context<Self>) {
        let Package {
            client_addr,
            envelope,
        } = package.clone();
        let Envelope {
            client_uuid,
            session_uuid,
            msg,
        } = envelope;

        use Msg::*;
        match msg {
            Ping => {}
            Pong => {}
            SessionClients(_) => {}
            SearchSuggestions(_) => {}
            QueryResults(_) => {}
            RegistorClient(_) => {
                self.all_session_clients
                    .entry(session_uuid)
                    .or_default()
                    .insert(client_uuid);
                self.all_clients.entry(client_uuid).or_insert(client_addr);
                let session_clients = self
                    .all_session_clients
                    .get(&session_uuid)
                    .expect("session clients");
                let msg = Msg::SessionClients(session_clients.clone());
                self.update_session_clients(session_uuid, msg);
            }
            UnRegistorClient(_) => {
                self.all_session_clients
                    .entry(session_uuid)
                    .or_default()
                    .remove(&client_uuid);
                self.all_clients.remove(&client_uuid);
                let session_clients = self
                    .all_session_clients
                    .get(&session_uuid)
                    .expect("session clients");
                let msg = Msg::SessionClients(session_clients.clone());
                self.update_session_clients(session_uuid, msg);
            }
            SearchText(_) => {
                self.search_addr.do_send(package);
            }
            QueryText(_) => {
                self.search_addr.do_send(package);
            }
        }
    }
}
