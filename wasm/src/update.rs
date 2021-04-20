use crate::media::*;
use crate::message::Message;
use crate::model::Model;
use rustgym_msg::ClientInfo;
use rustgym_msg::Msg;
use seed::{prelude::*, *};
use web_sys::MediaRecorder;

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
                Msg::RegistorClient(client_info) => {
                    if client_info.chrome {
                        orders.perform_cmd(async {
                            let media_stream = get_media_stream().await.expect("media stream");
                            Message::MediaStreamReady(media_stream)
                        });
                    }
                }
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
