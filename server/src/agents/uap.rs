use actix::prelude::*;
use actix_web::web::HttpRequest;

use actix_web::error::ErrorBadRequest;
use actix_web::Error;
use uaparser::{Parser, UserAgent, UserAgentParser};

#[derive(Message, Clone)]
#[rtype(result = "Option<UserAgent>")]
pub struct UserAgentRequest {
    uah: String,
}

impl UserAgentRequest {
    pub fn from_request(req: &HttpRequest) -> Result<Self, Error> {
        let header = req
            .headers()
            .get("User-Agent")
            .ok_or(1)
            .map_err(ErrorBadRequest)?;
        let uah = header.to_str().map_err(ErrorBadRequest)?.to_string();
        Ok(UserAgentRequest { uah })
    }
}

pub struct UapAgent {
    parser: UserAgentParser,
}

impl UapAgent {
    pub fn new() -> Self {
        let parser = UserAgentParser::from_yaml("uap-core/regexes.yaml").expect("Uap");
        UapAgent { parser }
    }
}

impl Actor for UapAgent {
    type Context = Context<Self>;
}

impl Handler<UserAgentRequest> for UapAgent {
    type Result = Option<UserAgent>;

    fn handle(&mut self, req: UserAgentRequest, _ctx: &mut Context<Self>) -> Self::Result {
        let ua = self.parser.parse_user_agent(&req.uah);
        Some(ua)
    }
}
