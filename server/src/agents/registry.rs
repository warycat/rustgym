use crate::agents::chunk::Chunk;
use crate::agents::envelope::Envelope;
use crate::agents::search::SearchAgent;
use crate::agents::websocket::SocketClient;
use actix::prelude::*;
use log::{error, info};
use rustgym_consts::*;
use rustgym_msg::ClientInfo;
use rustgym_msg::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::io::Write;
use std::process::{Child, Command, Stdio};
use std::rc::Rc;
use std::time::Duration;
use uuid::Uuid;

/*
    let ffmpeg = Command::new("ffmpeg")
        .args(&[
            "-i",
            "-",
            "-f",
            "hls",
            "-c:v",
            "copy",
            "-hls_time",
            "0.1",
            "-hls_flags",
            "delete_segments",
            &playlist_path_str,
        ])
        .stdin(Stdio::piped())
        .spawn()
        .expect("ffmpeg");
*/

#[derive(Clone)]
pub struct RegistryAgent {
    search_addr: Addr<SearchAgent>,
    all_clients: HashMap<Uuid, ClientInfo>,
    all_sockets: HashMap<Uuid, Addr<SocketClient>>,
    all_streams: HashMap<Uuid, Rc<RefCell<Child>>>,
}

impl RegistryAgent {
    pub fn new(search_addr: Addr<SearchAgent>) -> Self {
        let all_streams = HashMap::new();
        let all_clients = HashMap::new();
        let all_sockets = HashMap::new();

        RegistryAgent {
            search_addr,
            all_sockets,
            all_clients,
            all_streams,
        }
    }

    fn clients_with_session(&self, session_uuid: Uuid) -> Vec<ClientInfo> {
        let mut res: Vec<ClientInfo> = vec![];
        for client_info in self.all_clients.values() {
            if client_info.session_uuid == session_uuid {
                res.push(client_info.clone());
            }
        }
        res
    }

    fn all_clients(&self) -> Vec<ClientInfo> {
        self.all_clients.values().cloned().collect()
    }

    fn update_clients_with_session(&self, session_uuid: Uuid, msg_out: MsgOut) {
        for client_info in self.all_clients.values() {
            if client_info.session_uuid == session_uuid {
                if let Some(client_addr) = self.all_sockets.get(&client_info.client_uuid) {
                    let msg_out = msg_out.clone();
                    let client_addr_clone = client_addr.clone();
                    let envelope =
                        Envelope::from_msg_out(client_addr_clone, client_info.clone(), msg_out);
                    client_addr.do_send(envelope);
                } else {
                    error!("{} recipient not found", client_info.client_uuid);
                }
            }
        }
    }

    fn update_my_client(&self, client_uuid: Uuid, msg_out: MsgOut) {
        if let Some(client_info) = self.all_clients.get(&client_uuid) {
            if let Some(client_addr) = self.all_sockets.get(&client_uuid) {
                let msg_out = msg_out.clone();
                let client_addr_clone = client_addr.clone();
                let envelope =
                    Envelope::from_msg_out(client_addr_clone, client_info.clone(), msg_out);
                client_addr.do_send(envelope);
            } else {
                error!("{} recipient not found", client_info.client_uuid);
            }
        }
    }

    fn update_other_clients(&self, client_uuid: Uuid, msg_out: MsgOut) {
        for client_info in self.all_clients.values() {
            if client_info.client_uuid == client_uuid {
                continue;
            }
            if let Some(client_addr) = self.all_sockets.get(&client_info.client_uuid) {
                let msg_out = msg_out.clone();
                let client_addr_clone = client_addr.clone();
                let envelope =
                    Envelope::from_msg_out(client_addr_clone, client_info.clone(), msg_out);
                client_addr.do_send(envelope);
            } else {
                error!("{} recipient not found", client_info.client_uuid);
            }
        }
    }

    fn update_all_clients(&self, msg_out: MsgOut) {
        for client_info in self.all_clients.values() {
            if let Some(client_addr) = self.all_sockets.get(&client_info.client_uuid) {
                let msg_out = msg_out.clone();
                let client_addr_clone = client_addr.clone();
                let envelope =
                    Envelope::from_msg_out(client_addr_clone, client_info.clone(), msg_out);
                client_addr.do_send(envelope);
            } else {
                error!("{} recipient not found", client_info.client_uuid);
            }
        }
    }

    fn boardcast_chunk(&self, chunk: Chunk) {
        for socket in self.all_sockets.values() {
            socket.do_send(chunk.clone());
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
            client_info,
            msg,
        } = envelope.clone();
        let session_uuid = client_info.session_uuid;
        let client_uuid = client_info.client_uuid;

        match msg {
            Msg::In(msg_in) => match msg_in {
                MsgIn::SearchText(_) => {
                    self.search_addr.do_send(envelope);
                }
                MsgIn::QueryText(_) => {
                    self.search_addr.do_send(envelope);
                }
                MsgIn::StreamStart(mime_type) => {
                    // if mime_type == MIME_TYPE {
                    //     let webm_file = format!("{}/{}.webm", STREAM_DIR, client_uuid);
                    //     let ffmpeg = Command::new("ffmpeg")
                    //         .args(&[
                    //             "-i", "-", "-f", "webm", "-c:v", "copy", "-c:a", "copy", &webm_file,
                    //         ])
                    //         .stdin(Stdio::piped())
                    //         .spawn()
                    //         .expect("ffmpeg");
                    //     self.all_streams
                    //         .insert(client_uuid, Rc::new(RefCell::new(ffmpeg)));
                    //     ctx.run_later(Duration::from_secs(1), move |act, _| {
                    //         if let Some(client_info) = act.all_clients.get_mut(&client_uuid) {
                    //             client_info.streaming = true;
                    //         }
                    //         // let all_clients = act.all_clients();
                    //         // let msg_out = MsgOut::AllClients(all_clients);
                    //         // act.update_all_clients(msg_out);
                    //     });
                    // }
                }
                _ => {
                    error!("{:?}", msg_in);
                }
            },
            Msg::Out(msg_out) => match msg_out {
                MsgOut::RegistorClient(_) => {
                    let all_clients = self.all_clients();
                    let echo_msg = MsgOut::AllClients(all_clients);
                    self.update_my_client(client_info.client_uuid, echo_msg);
                    self.all_sockets.entry(client_uuid).or_insert(client_addr);
                    self.all_clients.entry(client_uuid).or_insert(client_info);
                    self.update_other_clients(client_uuid, msg_out);
                }
                MsgOut::UnRegistorClient(_) => {
                    // if let Some(ffmpeg) = self.all_streams.get(&client_uuid) {
                    //     ffmpeg.borrow_mut().kill().expect("command wasn't running");
                    // } else {
                    //     error!("stream not found");
                    // }
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

impl Handler<Chunk> for RegistryAgent {
    type Result = ();

    fn handle(&mut self, chunk: Chunk, _ctx: &mut Context<Self>) {
        let Chunk { client_info, bytes } = chunk.clone();
        let msg_bin: MsgBin = bincode::deserialize(&bytes).unwrap();
        let client_uuid = client_info.client_uuid;
        self.boardcast_chunk(chunk);
        info!("{} {}", client_uuid, bytes.len());
        // if let Some(ffmpeg) = self.all_streams.get(&client_uuid) {
        //     match ffmpeg
        //         .borrow()
        //         .stdin
        //         .as_ref()
        //         .unwrap()
        //         .write_all(&msg_bin.bytes)
        //     {
        //         Ok(_) => {}
        //         Err(e) => {
        //             error!("{}", e);
        //         }
        //     }
        // }
    }
}
