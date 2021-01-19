use std::time::{Duration, Instant};

use actix::prelude::*;
use actix_web::web;
use actix_web::Error;
use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web_actors::ws;
use log::info;
use rustgym_message::Message;

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

/// do websocket handshake and start `MyWebSocket` actor
pub async fn ws_index(r: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    info!("{:?}", r);
    let res = ws::start(MyWebSocket::new(), &r, stream);
    info!("{:?}", res);
    res
}

/// websocket connection is long running connection, it easier
/// to handle with an actor
struct MyWebSocket {
    /// Client must send ping at least once per 10 seconds (CLIENT_TIMEOUT),
    /// otherwise we drop connection.
    hb: Instant,
}

impl Actor for MyWebSocket {
    type Context = ws::WebsocketContext<Self>;

    /// Method is called on actor start. We start the heartbeat process here.
    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);
    }
}

/// Handler for `ws::Message`
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWebSocket {
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
                use Message::*;
                if let Ok(message) = serde_json::from_str::<Message>(&text) {
                    match message {
                        Ping => {
                            let pong = serde_json::to_string(&Message::Pong).expect("pong");
                            ctx.text(&pong)
                        }
                        Pong => {}
                        Login { name } => ctx.text(name),
                    }
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

impl MyWebSocket {
    fn new() -> Self {
        Self { hb: Instant::now() }
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
