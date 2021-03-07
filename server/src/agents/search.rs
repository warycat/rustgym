use crate::agents::envelope::Envelope;
use crate::agents::package::Package;
use actix::prelude::*;
use log::info;
use rustgym_consts::*;
use rustgym_msg::Msg;
use sonic_channel::*;

pub struct SearchAgent {
    search_channel: SearchChannel,
}

impl SearchAgent {
    pub fn new() -> Self {
        let search_channel = SearchChannel::start(SONIC_URL, SONIC_PASS).expect("channel");
        SearchAgent { search_channel }
    }
}

impl Actor for SearchAgent {
    type Context = Context<Self>;
}

impl Handler<Package> for SearchAgent {
    type Result = ();
    fn handle(&mut self, package: Package, _ctx: &mut Context<Self>) {
        let Package {
            client_addr,
            envelope,
        } = package.clone();
        let Envelope {
            client_uuid,
            session_uuid,
            msg,
        } = envelope;

        use Msg::*;
        match msg {
            Ping => {}
            Pong => {}
            SessionClients(_) => {}
            SearchSuggestions(_) => {}
            RegistorClient(_) => {}
            UnRegistorClient(_) => {}
            QueryResults(_) => {}
            SearchText(text) => {
                let text: String = text
                    .chars()
                    .map(|c| if c.is_ascii_alphabetic() { c } else { ' ' })
                    .collect();
                let search_words: Vec<String> =
                    text.split_whitespace().map(|s| s.to_string()).collect();
                if let Some(last) = search_words.last() {
                    info!("{}", last);
                    let mut suggestions = vec![];

                    match self
                        .search_channel
                        .suggest(SONIC_COLLECTION, SONIC_BUCKET, last)
                    {
                        Ok(words) => {
                            info!("{:?}", words);
                            for word in words {
                                let mut words = search_words.to_vec();
                                words.pop();
                                words.push(word);
                                let suggestion: String = words.join(" ");
                                suggestions.push(suggestion);
                            }
                        }
                        Err(err) => {
                            info!("{:?}", err);
                        }
                    }
                    let msg = Msg::SearchSuggestions(suggestions);
                    let envelope = Envelope {
                        client_uuid,
                        session_uuid,
                        msg,
                    };
                    let client_addr_clone = client_addr.clone();
                    let package = Package {
                        client_addr: client_addr_clone,
                        envelope,
                    };
                    client_addr.do_send(package);
                }
            }
            QueryText(text) => {}
        }
    }
}
