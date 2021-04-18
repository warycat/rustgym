use crate::message::Message;
use crate::model::Model;
use js_sys::{ArrayBuffer, Uint8Array};
use rustgym_consts::*;
use rustgym_msg::Msg;
use seed::{prelude::*, *};
use web_sys::{
    Blob, BlobEvent, Event, EventTarget, FileReader, MediaRecorder, MediaRecorderOptions,
    MediaStream,
};

pub fn update(msg: Message, model: &mut Model, orders: &mut impl Orders<Message>) {
    use Message::*;
    match msg {
        UrlChanged(subs::UrlChanged(url)) => {
            url.go_and_load();
        }
        SearchTextChanged(search_text) => {
            model.search_text = search_text.clone();
            if let Err(err) = model
                .web_socket
                .borrow()
                .send_json(&rustgym_msg::Msg::SearchText(search_text))
            {
                log!("error");
                orders.send_msg(WebSocketError(err));
            };
        }
        QueryText(search_text) => {
            model.search_text = search_text.clone();
            model.search_suggestions = vec![];
            if let Err(err) = model
                .web_socket
                .borrow()
                .send_json(&rustgym_msg::Msg::QueryText(search_text))
            {
                log!("error");
                orders.send_msg(WebSocketError(err));
            };
        }
        KeyDown(event) => {
            let key_code = event.key_code();
            if key_code == 13 {
                event.prevent_default();
                orders.send_msg(Message::QueryText(model.search_text.to_string()));
            }
        }
        WebSocketMsg(msg) => {
            log!(msg);
            match msg {
                Msg::Ping => {}
                Msg::Pong => {}
                Msg::RegistorClient(_) => {}
                Msg::UnRegistorClient(_) => {}
                Msg::SessionClients(_) => {}
                Msg::SearchText(_) => {}
                Msg::QueryText(_) => {}
                Msg::StreamStart(_) => {}
                Msg::SearchSuggestions(suggestions) => {
                    model.search_suggestions = suggestions;
                }
                Msg::QueryResults(mut query_results) => {
                    query_results.reverse();
                    model.query_results = query_results;
                }
            }
        }
        WebSocketError(err) => {
            model.web_socket_errors.push(err);
        }
        WebSocketClosed(close_event) => {
            log!("WebSocketClosed {:?}", close_event);
        }
        WebSocketOpened => {
            log!("WebSocketOpened");
        }
        WebSocketFailed => {
            log!("WebSocketFailed");
        }
        MediaStreamReady(media_stream) => {
            log!("MediaStreamReady");
            let media_recorder: MediaRecorder =
                media_recorder(&media_stream, model).expect("media recorder");
            model.media_stream = Some(media_stream);
        }
    }
}

fn media_recorder(media_stream: &MediaStream, model: &mut Model) -> Result<MediaRecorder, JsValue> {
    log!("media_recorder");
    let media_recorder: MediaRecorder = MediaRecorder::new_with_media_stream(media_stream)?;
    let wc = model.web_socket.clone();
    let onstart_cb = Closure::wrap(Box::new(move |event: Event| {
        let target: EventTarget = event.target().unwrap();
        let media_recorder: MediaRecorder = target.dyn_into().unwrap();
        let mime_type = media_recorder.mime_type();
        log!(mime_type);
        wc.borrow()
            .send_json(&rustgym_msg::Msg::StreamStart(mime_type))
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
            log!(bytes.len());
        }) as Box<dyn FnMut(Event)>);
        file_reader.set_onload(Some(onload.as_ref().unchecked_ref()));
        onload.forget();
    }) as Box<dyn FnMut(BlobEvent)>);
    media_recorder.set_ondataavailable(Some(ondataavailable_cb.as_ref().unchecked_ref()));
    ondataavailable_cb.forget();

    media_recorder.start_with_time_slice(TIME_SLICE).unwrap();
    Ok(media_recorder)
}
