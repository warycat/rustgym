use crate::agents::envelope::Envelope;
use crate::agents::registry::RegistryAgent;
use crate::agents::uap::{UapAgent, UserAgentRequest};
use crate::app_data::AppData;
use crate::session_data::update_session;
use actix::prelude::*;
use actix_session::Session;
use actix_web::web;
use actix_web::Error;
use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web_actors::ws;
use chrono::Utc;
use data_encoding::BASE64;
use log::debug;
use log::error;
use log::info;
use ring::hmac;
use rustgym_consts::*;
use rustgym_msg::ClientInfo;
use rustgym_msg::*;
use std::time::Instant;
use uuid::Uuid;

pub fn generate_turn_rest_api_cred(
    name: &str,
    turn_static_auth_secret: &str,
    ttl: i64,
) -> (String, String) {
    let now = Utc::now();
    let timestamp = now.timestamp() + ttl;
    let usercombo = format!("{}:{}", timestamp, name);
    let signed_key = hmac::Key::new(
        hmac::HMAC_SHA1_FOR_LEGACY_USE_ONLY,
        turn_static_auth_secret.as_bytes(),
    );
    let signature = hmac::sign(&signed_key, usercombo.as_bytes());
    let credential = BASE64.encode(signature.as_ref());
    (usercombo, credential)
}

pub async fn ws_index(
    data: web::Data<AppData>,
    req: HttpRequest,
    stream: web::Payload,
    session: Session,
    registry_addr: web::Data<Addr<RegistryAgent>>,
    uap_addr: web::Data<Addr<UapAgent>>,
) -> Result<HttpResponse, Error> {
    let uaq = UserAgentRequest::from_request(&req)?;
    let user_agent = if let Ok(Some(ua)) = uap_addr.get_ref().send(uaq).await {
        let family = ua.family;
        let major = ua.major;
        let minor = ua.minor;
        let patch = ua.patch;
        Some(rustgym_msg::UserAgent::new(family, major, minor, patch))
    } else {
        None
    };
    let session_data = update_session(session)?;
    let session_uuid = session_data.uuid;
    let name = session_data.name;
    let client_uuid = Uuid::new_v4();
    let (usercombo, credential) = generate_turn_rest_api_cred(
        &name,
        &data.turn_static_auth_secret.borrow().to_string(),
        3600 * 24,
    );
    let stun_server = IceServer::new(
        "stun:rustgym.com".to_string(),
        usercombo.clone(),
        credential.clone(),
    );
    let turn_server = IceServer::new(
        "turn:rustgym.com".to_string(),
        usercombo.clone(),
        credential.clone(),
    );
    let turns_server = IceServer::new(
        "turns:rustgym.com".to_string(),
        usercombo,
        credential,
    );
    let ice_servers = vec![stun_server, turn_server, turns_server];
    let client_info = ClientInfo {
        client_uuid,
        session_uuid,
        name,
        user_agent,
        ice_servers,
    };
    let socket_client = SocketClient::new(client_info, registry_addr.get_ref().clone());
    ws::start(socket_client, &req, stream)
}

#[derive(Clone)]
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
        let msg_out = MsgOut::RegistorClient(self.client_info.clone());
        let address = ctx.address();
        let envelop = Envelope::from_msg_out(address, self.client_info.client_uuid, msg_out);
        self.registry_addr.do_send(envelop);
        self.hb(ctx);
    }

    fn stopping(&mut self, ctx: &mut Self::Context) -> actix::Running {
        let msg_out = MsgOut::UnRegistorClient(self.client_info.clone());
        let address = ctx.address();
        let envelop = Envelope::from_msg_out(address, self.client_info.client_uuid, msg_out);
        self.registry_addr.do_send(envelop);
        actix::Running::Stop
    }
}

impl Handler<Envelope> for SocketClient {
    type Result = ();

    fn handle(&mut self, envelope: Envelope, ctx: &mut Self::Context) {
        match envelope.msg {
            Msg::In(msg_in) => {
                error!("{:?}", msg_in);
            }
            Msg::Out(msg_out) => {
                let json = serde_json::to_string(&msg_out).expect("json");
                ctx.text(json);
            }
        }
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
                if let Ok(msg_in) = serde_json::from_str::<MsgIn>(&text) {
                    info!("{:?}", msg_in);
                    let address = ctx.address();
                    let envelope =
                        Envelope::from_msg_in(address, self.client_info.client_uuid, msg_in);
                    self.registry_addr.do_send(envelope);
                } else {
                    info!("{:?}", text);
                    ctx.text(text)
                }
            }
            Ok(ws::Message::Binary(_)) => {}
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => ctx.stop(),
        }
    }
}
