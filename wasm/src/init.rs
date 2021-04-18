use crate::message::Message;
use crate::model::Model;
use seed::{prelude::*, *};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen_futures::JsFuture;

use web_sys::{
    Document, HtmlVideoElement, Location, MediaDevices, MediaSource, MediaStream,
    MediaStreamConstraints, Navigator, Url, Window,
};

fn window() -> Window {
    web_sys::window().expect("no global `window` exists")
}

fn document() -> Document {
    window().document().unwrap()
}

fn navigator() -> Navigator {
    window().navigator()
}

async fn get_media_stream() -> Result<MediaStream, JsValue> {
    let navigator: Navigator = navigator();
    let media_devices: MediaDevices = navigator.media_devices()?;
    let mut constraints = MediaStreamConstraints::new();
    constraints.audio(&JsValue::from_bool(true));
    constraints.video(&JsValue::from_bool(true));
    let get_user_media_promise = media_devices.get_user_media_with_constraints(&constraints)?;
    let media_stream: MediaStream = JsFuture::from(get_user_media_promise).await?.dyn_into()?;
    Ok(media_stream)
}

fn media_source(media_stream: &MediaStream) -> Result<MediaSource, JsValue> {
    let media_source: MediaSource = MediaSource::new()?;
    let video: HtmlVideoElement = document().query_selector("video")?.unwrap().dyn_into()?;
    let url = Url::create_object_url_with_source(&media_source);
    video.set_src_object(Some(media_stream));
    // video.play();
    log!(url);
    Ok(media_source)
}

fn wsurl() -> Result<String, JsValue> {
    let window: Window = window();
    let location: Location = window.location();
    let protocol: String = location.protocol()?;
    let host: String = location.host()?;
    let ws_protocol = if protocol == "https:" {
        "wss://"
    } else {
        "ws://"
    };
    Ok(format!("{}{}/ws/", ws_protocol, host))
}

fn web_socket(orders: &mut impl Orders<Message>) -> WebSocket {
    WebSocket::builder(wsurl().expect("url"), orders)
        .on_open(|| Message::WebSocketOpened)
        .on_message(
            |msg: WebSocketMessage| match msg.json::<rustgym_msg::Msg>() {
                Ok(msg) => Message::WebSocketMsg(msg),
                Err(err) => Message::WebSocketError(err),
            },
        )
        .on_close(Message::WebSocketClosed)
        .on_error(|| Message::WebSocketFailed)
        .build_and_open()
        .expect("web_socket")
}

pub fn init(url: seed::Url, orders: &mut impl Orders<Message>) -> Model {
    orders.subscribe(Message::UrlChanged);
    let navigator = navigator();
    log!(navigator.user_agent());
    orders.perform_cmd(async {
        Message::MediaStreamReady(get_media_stream().await.expect("media stream"))
    });
    // let media_stream: MediaStream = join!(async { media_stream().await.expect("media stream") }).0;
    // spawn_local(
    //     // let media_source = media_source(&media_stream).await.unwrap();
    // });
    // let media_recorder = media_recorder(&media_stream).expect("media recorder");

    let web_socket = Rc::new(RefCell::new(web_socket(orders)));
    let search_text = "".to_string();
    let web_socket_errors = vec![];
    let search_suggestions = vec![];
    let query_results = vec![];
    let media_stream = None;
    Model {
        search_text,
        search_suggestions,
        query_results,
        web_socket,
        web_socket_errors,
        media_stream,
    }
}
