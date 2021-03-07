use crate::message::Message;
use crate::model::Model;
use rustgym_msg::Msg;
use seed::{prelude::*, *};

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
                .send_json(&rustgym_msg::Msg::SearchText(search_text))
            {
                console_log!("error");
                orders.send_msg(WebSocketError(err));
            };
        }
        QueryText(search_text) => {
            model.search_text = search_text;
            model.suggestions = vec![];
        }
        KeyDown(event) => {
            let key_code = event.key_code();
            if key_code == 13 {
                event.prevent_default();
                orders.send_msg(Message::QueryText(model.search_text.to_string()));
            }
        }
        WebSocketMsg(msg) => {
            console_log!("{:?}", msg);
            match msg {
                Msg::Ping => {}
                Msg::Pong => {}
                Msg::RegistorClient(client_info) => {
                    console_log!("{:?}", client_info);
                }
                Msg::UnRegistorClient(client_info) => {
                    console_log!("{:?}", client_info);
                }
                Msg::SessionClients(session_clients) => {
                    console_log!("{:?}", session_clients);
                }
                Msg::SearchText(search_text) => {
                    console_log!("{:?}", search_text);
                }
                Msg::SearchSuggestions(suggestions) => {
                    console_log!("{:?}", suggestions);
                    model.suggestions = suggestions;
                }
                Msg::QueryText(text) => {
                    console_log!("{:?}", text);
                }
                Msg::QueryResults(results) => {
                    model.results = results;
                }
            }
        }
        WebSocketError(err) => {
            model.web_socket_errors.push(err);
        }
        WebSocketOpened => {
            model.web_socket.send_text("Ping").expect("Ping");
        }
        WebSocketClosed(_) => {
            // model.connected = false;
        }
        WebSocketFailed => {
            console_log!("WebSocketFailed");
        }
    }
}
