use crate::message::Message;
use crate::model::Model;
use seed::{prelude::*, *};
use wasm_bindgen_futures::JsFuture;

use web_sys::{
    Blob, BlobEvent, Document, Event, HtmlVideoElement, Location, MediaDevices, MediaRecorder,
    MediaSource, MediaStream, MediaStreamConstraints, Navigator, RecordingState, Url, Window,
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

async fn media_stream() -> Result<MediaStream, JsValue> {
    let navigator: Navigator = navigator();
    let media_devices: MediaDevices = navigator.media_devices()?;
    let mut constraints = MediaStreamConstraints::new();
    constraints.audio(&JsValue::from_bool(true));
    constraints.video(&JsValue::from_bool(true));
    let get_user_media_promise = media_devices.get_user_media_with_constraints(&constraints)?;
    let media_stream: MediaStream = JsFuture::from(get_user_media_promise).await?.dyn_into()?;
    Ok(media_stream)
}

async fn media_recorder(media_stream: &MediaStream) -> Result<MediaRecorder, JsValue> {
    let media_recorder: MediaRecorder = MediaRecorder::new_with_media_stream(media_stream)?;
    let ondataavailable_cb = Closure::wrap(Box::new(move |event: BlobEvent| {
        let blob = event.data().unwrap();
        log!(blob);
    }) as Box<dyn FnMut(BlobEvent)>);
    media_recorder.set_ondataavailable(Some(ondataavailable_cb.as_ref().unchecked_ref()));
    ondataavailable_cb.forget();
    media_recorder.start_with_time_slice(1000).unwrap();
    Ok(media_recorder)
}

async fn media_source(media_stream: &MediaStream) -> Result<MediaSource, JsValue> {
    let media_source: MediaSource = MediaSource::new()?;
    let video: HtmlVideoElement = document().query_selector("video")?.unwrap().dyn_into()?;
    let url = Url::create_object_url_with_source(&media_source);
    video.set_src_object(Some(media_stream));
    video.play();
    log!(url);
    Ok(media_source)
}

// async fn source_url(media_source: &MediaSource) -> Result<String, JsValue> {
//     let video: HtmlVideoElement = document().query_selector("video")?.unwrap().dyn_into()?;
//     Url::create_object_url_with_source(media_source)
// }

//     let media_source: MediaSource = MediaSource::new().unwrap();
//     log!(media_recorder);
//     // Ok::<(), JsValue>(())

// let types = vec![
//     "video/webm;codecs=vp8",
//     "video/webm;codecs=vp9",
//     "video/webm;codecs=h264",
//     "audio/webm;codecs=opus",
//     "video/x-matroska;codecs=avc1,opus",
// ];

// for t in types {
//     log!(t, MediaRecorder::is_type_supported(t));
// }

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

pub fn init(url: seed::Url, orders: &mut impl Orders<Message>) -> Model {
    orders.subscribe(Message::UrlChanged);
    spawn_local(async {
        let media_stream = media_stream().await.unwrap();
        // let media_recorder = media_recorder(&media_stream).await.unwrap();
        let media_source = media_source(&media_stream).await.unwrap();
    });
    let web_socket = WebSocket::builder(wsurl().expect("url"), orders)
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
        .expect("web_socket");
    let mut search_text = "".to_string();
    if let Some(v) = url.search().get("text") {
        if let Some(first) = v.first() {
            search_text = first.to_string();
        }
    }
    let web_socket_errors = vec![];
    let search_suggestions = vec![];
    let query_results = vec![];
    Model {
        search_text,
        search_suggestions,
        query_results,
        web_socket,
        web_socket_errors,
    }
}
