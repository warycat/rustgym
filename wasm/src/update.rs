use crate::device::*;
use crate::media::*;
use crate::message::Message;
use crate::model::Model;
use crate::websocket::*;
use rustgym_msg::*;
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
                .send_json(&rustgym_msg::MsgIn::SearchText(search_text))
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
                .send_json(&rustgym_msg::MsgIn::QueryText(search_text))
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
        WebSocketMsgOut(msg_out) => {
            log!(msg_out);
            match msg_out {
                MsgOut::RegistorClient(client_info) => {
                    if let Some(user_agent) = client_info.user_agent.as_ref() {
                        log!(user_agent.detect_device());
                    }
                    // let uuid = client_info.client_uuid;
                    if model.client_info.is_none() {
                        // if client_info.chrome {
                        //     orders.perform_cmd(async {
                        //         let media_stream = get_media_stream().await.expect("media stream");
                        //         Message::MediaStreamReady(media_stream)
                        //     });
                        // }
                        model.client_info = Some(client_info);
                    }
                    // let msg_sender = orders.msg_sender();
                    // model
                    //     .all_source_urls
                    //     .entry(uuid)
                    //     .or_insert(media_source_url(uuid, msg_sender).expect("media source url"));
                }
                MsgOut::UnRegistorClient(client_info) => {
                    model.all_source_urls.remove(&client_info.client_uuid);
                    model.all_source_buffers.remove(&client_info.client_uuid);
                }
                MsgOut::SessionClients(_) => {}
                MsgOut::SearchSuggestions(suggestions) => {
                    model.search_suggestions = suggestions;
                }
                MsgOut::QueryResults(mut query_results) => {
                    query_results.reverse();
                    model.query_results = query_results;
                }
                MsgOut::AllClients(all_clients) => {
                    orders.send_msg(Message::AllClients(all_clients));
                }
                _ => {
                    log!("error", msg_out);
                }
            }
        }
        WebSocketMsgBin(mut msg_bin) => {
            if let Some(source_buffer) = model.all_source_buffers.get_mut(&msg_bin.uuid) {
                match source_buffer.append_buffer_with_u8_array(&mut msg_bin.bytes) {
                    Ok(_) => {
                        log!("ok", msg_bin.uuid);
                    }
                    Err(err) => {
                        log!("err", msg_bin.uuid, err);
                    }
                }
            }
        }
        WebSocketError(err) => {
            model.web_socket_errors.push(err);
        }
        WebSocketClosed(close_event) => {
            log!("WebSocketClosed {:?}", close_event);
            model.reconnect(web_socket_builder(orders));
        }
        WebSocketOpened => {
            log!("WebSocketOpened");
        }
        WebSocketFailed => {
            log!("WebSocketFailed");
        }
        MediaStreamReady(media_stream) => {
            log!("MediaStreamReady");
            let uuid = model.client_info.as_ref().unwrap().client_uuid;
            let media_recorder: MediaRecorder =
                media_recorder(uuid, &media_stream, model.web_socket.clone())
                    .expect("media recorder");
            model.media_recorder = Some(media_recorder);
            model.media_stream = Some(media_stream);
        }
        SourceBuffer(uuid, source_buffer) => {
            model
                .all_source_buffers
                .entry(uuid)
                .or_insert(source_buffer);
        }
        AllClients(all_clients) => {
            let uuid = model.client_info.as_ref().unwrap().client_uuid;
            for client in all_clients.iter() {
                let msg_sender = orders.msg_sender();
                model
                    .all_source_urls
                    .entry(client.client_uuid)
                    .or_insert(media_source_url(uuid, msg_sender).expect("media source url"));
            }
            model.all_clients = all_clients;
        }
    }
}
