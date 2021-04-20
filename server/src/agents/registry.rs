use crate::agents::chunk::Chunk;
use crate::agents::envelope::Envelope;
use crate::agents::package::Package;
use crate::agents::search::SearchAgent;
use crate::agents::websocket::SocketClient;
use actix::prelude::*;
use log::{error, info};
use rustgym_consts::*;
use rustgym_msg::ClientInfo;
use rustgym_msg::Msg;
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::process::Command;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::Mutex;
use uuid::Uuid;

#[derive(Clone)]
pub struct RegistryAgent {
    search_addr: Addr<SearchAgent>,
    all_session_clients: HashMap<Uuid, HashSet<ClientInfo>>,
    all_clients: HashMap<Uuid, Addr<SocketClient>>,
    all_streams: HashMap<Uuid, Rc<RefCell<File>>>,
}

impl RegistryAgent {
    pub fn new(search_addr: Addr<SearchAgent>) -> Self {
        let all_session_clients = HashMap::new();
        let all_clients = HashMap::new();
        let all_streams = HashMap::new();

        RegistryAgent {
            search_addr,
            all_session_clients,
            all_clients,
            all_streams,
        }
    }

    fn update_session_clients(&self, session_uuid: Uuid, msg: Msg) {
        if let Some(session_clients) = self.all_session_clients.get(&session_uuid) {
            for client_info in session_clients.iter() {
                if let Some(client_addr) = self.all_clients.get(&client_info.client_uuid) {
                    let msg = msg.clone();
                    let client_addr_clone = client_addr.clone();
                    let envelope = Envelope::new(client_info.clone(), msg);
                    let package = Package::new(client_addr_clone, envelope);
                    client_addr.do_send(package);
                } else {
                    error!("{} recipient not found", client_info.client_uuid);
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
        let Envelope { client_info, msg } = envelope;
        use Msg::*;
        match msg {
            Ping => {}
            Pong => {}
            SessionClients(_) => {}
            SearchSuggestions(_) => {}
            QueryResults(_) => {}
            RegistorClient(_) => {
                let session_uuid = client_info.session_uuid;
                let client_uuid = client_info.client_uuid;
                self.all_session_clients
                    .entry(client_info.session_uuid)
                    .or_default()
                    .insert(client_info);
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
                    .entry(client_info.session_uuid)
                    .or_default()
                    .remove(&client_info);
                self.all_clients.remove(&client_info.client_uuid);
                let session_clients = self
                    .all_session_clients
                    .get(&client_info.session_uuid)
                    .expect("session clients");
                let msg = Msg::SessionClients(session_clients.clone());
                let client_uuid = client_info.client_uuid;
                self.all_streams.remove(&client_uuid);
                self.update_session_clients(client_info.session_uuid, msg);
            }
            SearchText(_) => {
                self.search_addr.do_send(package);
            }
            QueryText(_) => {
                self.search_addr.do_send(package);
            }
            StreamStart(mime_type) => {
                let client_uuid = client_info.client_uuid;
                if mime_type == MIME_TYPE {
                    let file_name = format!("stream/{}.{}", client_uuid, "webm");
                    let file = OpenOptions::new()
                        .write(true)
                        .create(true)
                        .open(file_name)
                        .expect("file");
                    self.all_streams
                        .entry(client_uuid)
                        .or_insert(Rc::new(RefCell::new(file)));
                }

                // let ffplay = Command::new("ffplay")
                //     .args(&["-f", vals[0], "-i", "0"])
                //     .output()
                //     .expect("ffplay");
                // println!("{} {:?}", mime_type, ffplay);
            }
        }
    }
}

impl Handler<Chunk> for RegistryAgent {
    type Result = ();

    fn handle(&mut self, chunk: Chunk, _ctx: &mut Context<Self>) {
        let Chunk {
            client_addr,
            client_info,
            bytes,
        } = chunk.clone();
        if let Some(file) = self.all_streams.get(&client_info.client_uuid) {
            match file.borrow_mut().write_all(&bytes) {
                Ok(_) => {
                    info!("{}", bytes.len())
                }
                Err(e) => {
                    error!("{}", e);
                }
            }
        }
    }
}
