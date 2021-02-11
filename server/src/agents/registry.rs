use crate::agents::envelope::Envelope;
use crate::agents::package::Package;
use crate::agents::websocket::SocketClient;
use actix::prelude::*;
use log::error;
use rustgym_msg::Msg;
use std::collections::HashMap;
use std::collections::HashSet;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct RegistryAgent {
    all_session_clients: HashMap<Uuid, HashSet<Uuid>>,
    all_clients: HashMap<Uuid, Addr<SocketClient>>,
}

impl RegistryAgent {
    pub fn new() -> Self {
        let all_session_clients = HashMap::new();
        let all_clients = HashMap::new();
        RegistryAgent {
            all_session_clients,
            all_clients,
        }
    }
    fn update_session_clients(&self, session_uuid: Uuid, session_clients: HashSet<Uuid>) {
        for &client_uuid in &session_clients {
            if let Some(client_addr) = self.all_clients.get(&client_uuid) {
                let client_addr_clone = client_addr.clone();
                let msg = Msg::SessionClients(session_clients.clone());
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
    }
}

impl Actor for RegistryAgent {
    type Context = Context<Self>;
}

impl Handler<Package> for RegistryAgent {
    type Result = ();

    fn handle(&mut self, package: Package, _: &mut Context<Self>) {
        let Package {
            client_addr,
            envelope,
        } = package;
        let Envelope {
            client_uuid,
            session_uuid,
            msg,
        } = envelope;

        use Msg::*;
        match msg {
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
                self.update_session_clients(session_uuid, session_clients.clone());
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
                self.update_session_clients(session_uuid, session_clients.clone());
            }
            _ => {}
        }
    }
}
