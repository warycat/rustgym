use crate::message::Message;
use crate::model::Model;
use crate::websocket::*;
use seed::prelude::*;

pub fn init(_url: seed::Url, orders: &mut impl Orders<Message>) -> Model {
    orders.subscribe(Message::UrlChanged);
    Model::new(web_socket_builder(orders))
}
