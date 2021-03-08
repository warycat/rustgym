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
            model.search_text = search_text.clone();
            model.search_suggestions = vec![];
            if let Err(err) = model
                .web_socket
                .send_json(&rustgym_msg::Msg::QueryText(search_text))
            {
                console_log!("error");
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
            console_log!("{:?}", msg);
            match msg {
                Msg::Ping => {}
                Msg::Pong => {}
                Msg::RegistorClient(_) => {}
                Msg::UnRegistorClient(_) => {}
                Msg::SessionClients(_) => {}
                Msg::SearchText(_) => {}
                Msg::QueryText(_) => {}
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
            console_log!("WebSocketClosed {:?}", close_event);
        }
        WebSocketOpened => {
            console_log!("WebSocketOpened");
        }
        WebSocketFailed => {
            console_log!("WebSocketFailed");
        }
    }
}
