use actix::prelude::*;
use actix_web::web::Bytes;
use rustgym_msg::ClientInfo;

#[derive(Message, Clone, new)]
#[rtype(result = "()")]
pub struct Chunk {
    pub client_info: ClientInfo,
    pub bytes: Bytes,
}
