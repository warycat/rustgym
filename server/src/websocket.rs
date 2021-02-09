use crate::session_data::update_session;
use actix::prelude::*;
use actix_session::Session;
use actix_web::web;
use actix_web::Error;
use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web_actors::ws;
use log::info;
use rustgym_msg::Msg;
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};
use uuid::Uuid;

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

#[derive(Message, Debug, Deserialize, Serialize)]
#[rtype(result = "()")]
struct Envelope {
    client_uuid: Uuid,
    session_uuid: Uuid,
    msg: Msg,
}

/// do websocket handshake and start `MyWebSocket` actor
pub async fn ws_index(
    r: HttpRequest,
    stream: web::Payload,
    session: Session,
) -> Result<HttpResponse, Error> {
    let session_data = update_session(session)?;
    let session_uuid = session_data.uuid;
    info!("{:?}", r);
    let res = ws::start(SocketClient::new(session_uuid), &r, stream);
    info!("{:?}", res);
    res
}

struct SocketClient {
    uuid: Uuid,
    hb: Instant,
    session_uuid: Uuid,
}

impl SocketClient {
    fn new(session_uuid: Uuid) -> Self {
        let uuid = Uuid::new_v4();
        let hb = Instant::now();
        Self {
            uuid,
            hb,
            session_uuid,
        }
    }

    fn hb(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                info!("Websocket Client heartbeat failed, disconnecting!");
                ctx.stop();
                return;
            }

            ctx.ping(b"");
        });
    }
}

impl Actor for SocketClient {
    type Context = ws::WebsocketContext<Self>;

    /// Method is called on actor start. We start the heartbeat process here.
    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);
    }
}

/// Handler for `ws::Message`
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for SocketClient {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        // process websocket messages
        info!("WS: {:?}", msg);
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
                    let envelope = Envelope {
                        client_uuid: self.uuid,
                        session_uuid: self.session_uuid,
                        msg,
                    };
                    println!("{:?}", envelope);
                } else {
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
