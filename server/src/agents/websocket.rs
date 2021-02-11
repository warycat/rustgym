// use crate::envelope::Envelope;
use crate::agents::envelope::Envelope;
use crate::agents::package::Package;
use crate::agents::registry::RegistryAgent;
use crate::session_data::update_session;
use actix::prelude::*;
use actix_session::Session;
use actix_web::web;
use actix_web::Error;
use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web_actors::ws;
use log::debug;
use log::info;
use rustgym_consts::*;
use rustgym_msg::ClientInfo;
use rustgym_msg::Msg;
use std::time::Instant;
use uuid::Uuid;

pub async fn ws_index(
    r: HttpRequest,
    stream: web::Payload,
    session: Session,
    registry_addr: web::Data<Addr<RegistryAgent>>,
) -> Result<HttpResponse, Error> {
    let session_data = update_session(session)?;
    let session_uuid = session_data.uuid;
    let name = session_data.name;
    let client_uuid = Uuid::new_v4();
    let client_info = ClientInfo {
        client_uuid,
        session_uuid,
        name,
    };
    ws::start(
        SocketClient::new(client_info, registry_addr.get_ref().clone()),
        &r,
        stream,
    )
}

#[derive(Debug, Clone)]
pub struct SocketClient {
    hb: Instant,
    client_info: ClientInfo,
    registry_addr: Addr<RegistryAgent>,
}

impl SocketClient {
    fn new(client_info: ClientInfo, registry_addr: Addr<RegistryAgent>) -> Self {
        let hb = Instant::now();
        Self {
            hb,
            client_info,
            registry_addr,
        }
    }

    fn make_envelope(&self, msg: Msg) -> Envelope {
        let session_uuid = self.client_info.session_uuid;
        let client_uuid = self.client_info.client_uuid;
        Envelope {
            session_uuid,
            client_uuid,
            msg,
        }
    }

    fn make_package(&self, ctx: &mut <Self as Actor>::Context, envelope: Envelope) -> Package {
        let client_addr = ctx.address();
        Package {
            client_addr,
            envelope,
        }
    }

    fn hb(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                info!("Websocket Client heartbeat failed, disconnecting!");
                ctx.stop();
                return;
            }
            ctx.ping(act.client_info.client_uuid.as_bytes());
        });
    }
}

impl Actor for SocketClient {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let msg = Msg::RegistorClient(self.client_info.clone());
        let json = serde_json::to_string(&msg).expect("json");
        ctx.text(json);
        let envelope = self.make_envelope(msg);
        let package = self.make_package(ctx, envelope);
        self.registry_addr.do_send(package);
        self.hb(ctx);
    }

    fn stopping(&mut self, ctx: &mut Self::Context) -> actix::Running {
        let msg = Msg::UnRegistorClient(self.client_info.clone());
        let envelope = self.make_envelope(msg);
        let package = self.make_package(ctx, envelope);
        self.registry_addr.do_send(package);
        self.hb(ctx);
        actix::Running::Stop
    }
}

impl Handler<Package> for SocketClient {
    type Result = ();

    fn handle(&mut self, package: Package, ctx: &mut Self::Context) {
        let Package { envelope, .. } = package;
        let Envelope { msg, .. } = envelope;
        let json = serde_json::to_string(&msg).expect("json");
        ctx.text(json);
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for SocketClient {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        debug!("WS: {:?}", msg);
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            }
            Ok(ws::Message::Text(text)) => {
                if let Ok(msg) = serde_json::from_str::<Msg>(&text) {
                    info!("{:?}", msg);
                    let envelope = self.make_envelope(msg);
                    let package = self.make_package(ctx, envelope);
                    self.registry_addr.do_send(package);
                } else {
                    info!("{:?}", text);
                    ctx.text(text)
                }
            }
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => ctx.stop(),
        }
    }
}
