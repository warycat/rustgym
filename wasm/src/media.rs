use crate::model::Model;

use js_sys::{ArrayBuffer, Uint8Array};
use rustgym_consts::*;
use seed::{prelude::*, *};
use wasm_bindgen_futures::JsFuture;
use web_sys::{
    Blob, BlobEvent, Event, EventTarget, FileReader, HtmlVideoElement, MediaDevices, MediaRecorder,
    MediaRecorderOptions, MediaSource, MediaStream, MediaStreamConstraints, Navigator, Url,
};

pub async fn get_media_stream() -> Result<MediaStream, JsValue> {
    let navigator: Navigator = window().navigator();
    let media_devices: MediaDevices = navigator.media_devices()?;
    let mut constraints = MediaStreamConstraints::new();
    constraints.audio(&JsValue::from_bool(true));
    constraints.video(&JsValue::from_bool(true));
    let get_user_media_promise = media_devices.get_user_media_with_constraints(&constraints)?;
    let media_stream: MediaStream = JsFuture::from(get_user_media_promise).await?.dyn_into()?;
    Ok(media_stream)
}

pub fn media_source(media_stream: &MediaStream) -> Result<MediaSource, JsValue> {
    let media_source: MediaSource = MediaSource::new()?;
    let video: HtmlVideoElement = document().query_selector("video")?.unwrap().dyn_into()?;
    let url = Url::create_object_url_with_source(&media_source);
    video.set_src_object(Some(media_stream));
    // video.play();
    log!(url);
    Ok(media_source)
}

pub fn media_recorder(
    media_stream: &MediaStream,
    model: &mut Model,
) -> Result<MediaRecorder, JsValue> {
    log!("media_recorder");
    let mut options = MediaRecorderOptions::new();
    options.mime_type(MIME_TYPE);
    let media_recorder: MediaRecorder =
        MediaRecorder::new_with_media_stream_and_media_recorder_options(media_stream, &options)?;
    let wc = model.web_socket.clone();
    let onstart_cb = Closure::wrap(Box::new(move |event: Event| {
        let target: EventTarget = event.target().unwrap();
        let media_recorder: MediaRecorder = target.dyn_into().unwrap();
        let mime_type = media_recorder.mime_type();
        log!(mime_type);
        wc.borrow()
            .send_json(&rustgym_msg::MsgIn::StreamStart(mime_type))
            .expect("strart stream");
    }) as Box<dyn FnMut(Event)>);
    media_recorder.set_onstart(Some(onstart_cb.as_ref().unchecked_ref()));
    onstart_cb.forget();

    let wc = model.web_socket.clone();
    let ondataavailable_cb = Closure::wrap(Box::new(move |event: BlobEvent| {
        let blob: Blob = event.data().unwrap();
        let wcc = wc.clone();
        let file_reader = web_sys::FileReader::new().unwrap();
        file_reader.read_as_array_buffer(&blob).unwrap();
        let onload = Closure::wrap(Box::new(move |event: Event| {
            let file_reader: FileReader = event.target().unwrap().dyn_into().unwrap();
            let buf: ArrayBuffer = file_reader.result().unwrap().dyn_into().unwrap();
            let arr: Uint8Array = Uint8Array::new(&buf);
            let size = buf.byte_length();
            let mut bytes = vec![0; size as usize];
            arr.copy_to(&mut bytes);
            wcc.borrow().send_bytes(&bytes).expect("send bytes");
        }) as Box<dyn FnMut(Event)>);
        file_reader.set_onload(Some(onload.as_ref().unchecked_ref()));
        onload.forget();
    }) as Box<dyn FnMut(BlobEvent)>);
    media_recorder.set_ondataavailable(Some(ondataavailable_cb.as_ref().unchecked_ref()));
    ondataavailable_cb.forget();

    media_recorder.start_with_time_slice(TIME_SLICE).unwrap();
    Ok(media_recorder)
}
