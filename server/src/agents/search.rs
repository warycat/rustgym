use crate::agents::envelope::Envelope;
use crate::db::*;
use actix::prelude::*;
use anyhow::Result;
use diesel::prelude::*;
use log::error;
use log::info;
use rustgym_consts::*;
use rustgym_msg::QueryResult;
use rustgym_msg::*;
use rustgym_schema::AdventOfCodeDescription;
use rustgym_schema::GoogleProblem;
use rustgym_schema::LeetcodeQuestion;
use sonic_channel::*;

pub struct SearchAgent {
    pool: SqlitePool,
    search_channel: SearchChannel,
}

impl SearchAgent {
    pub fn new(pool: SqlitePool) -> Self {
        let search_channel = SearchChannel::start(SONIC_URL, SONIC_PASS).expect("channel");
        SearchAgent {
            pool,
            search_channel,
        }
    }

    pub fn reconnect(&mut self) {
        self.search_channel = SearchChannel::start(SONIC_URL, SONIC_PASS).expect("channel");
    }
}

impl Actor for SearchAgent {
    type Context = Context<Self>;
}

impl Handler<Envelope> for SearchAgent {
    type Result = ();
    fn handle(&mut self, envelope: Envelope, _ctx: &mut Context<Self>) {
        let Envelope {
            client_addr,
            client_uuid,
            msg,
        } = envelope;

        let dest = Dest::col_buc(SONIC_COLLECTION, SONIC_BUCKET);

        match msg {
            Msg::In(msg_in) => match msg_in {
                MsgIn::SearchText(text) => {
                    let text: String = cleanup(text);
                    let search_words: Vec<String> =
                        text.split_whitespace().map(|s| s.to_string()).collect();
                    let mut suggestions = vec![];
                    if let Some(last) = search_words.last() {
                        info!("{}", last);
                        match self.search_channel.suggest(SuggestRequest::new(dest, last)) {
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
                                info!("reconnect {:?}", err);
                                self.reconnect();
                            }
                        }
                    }
                    let msg_out = MsgOut::SearchSuggestions(suggestions);
                    let envelope =
                        Envelope::from_msg_out(client_addr.clone(), client_uuid, msg_out);
                    client_addr.do_send(envelope);
                }
                MsgIn::QueryText(text) => {
                    let text: String = cleanup(text);
                    let mut query_results: Vec<QueryResult> = vec![];
                    match self.search_channel.query(QueryRequest::new(dest, &text)) {
                        Ok(objects) => {
                            if let Ok(results) = get_results(objects, &self.pool) {
                                query_results = results;
                            }
                        }
                        Err(err) => {
                            info!("reconnect {:?}", err);
                            self.reconnect();
                        }
                    }
                    let msg_out = MsgOut::QueryResults(query_results);
                    let envelope =
                        Envelope::from_msg_out(client_addr.clone(), client_uuid, msg_out);
                    client_addr.do_send(envelope);
                }
                _ => {
                    error!("{:?}", msg_in);
                }
            },
            Msg::Out(msg_out) => {
                error!("{:?}", msg_out);
            }
        }
    }
}

fn cleanup(text: String) -> String {
    text.chars()
        .map(|c| if c.is_ascii_alphabetic() { c } else { ' ' })
        .collect()
}

fn get_results(objects: Vec<String>, pool: &SqlitePool) -> Result<Vec<QueryResult>> {
    use rustgym_schema::schema::adventofcode_description::dsl::*;
    use rustgym_schema::schema::google_problem::dsl::*;
    use rustgym_schema::schema::leetcode_question::dsl::*;
    let mut res = vec![];
    let conn = pool.get()?;
    for object in objects {
        let parts: Vec<&str> = object.split('_').collect();
        match parts[0] {
            "leetcode" => {
                let id_ = parts[1].parse::<i32>()?;
                let question: LeetcodeQuestion = leetcode_question
                    .filter(rustgym_schema::schema::leetcode_question::dsl::id.eq(id_))
                    .first(&conn)?;
                let query_result = QueryResult::new(
                    parts[1].to_string(),
                    question.title.to_string(),
                    question.href(),
                    question.from(),
                );
                res.push(query_result);
            }
            "adventofcode" => {
                let id_ = parts[1].parse::<i32>()?;
                let description: AdventOfCodeDescription = adventofcode_description
                    .filter(rustgym_schema::schema::adventofcode_description::dsl::id.eq(id_))
                    .first(&conn)?;
                let query_result = QueryResult::new(
                    parts[1].to_string(),
                    description.title.to_string(),
                    description.href(),
                    description.from(),
                );
                res.push(query_result);
            }
            "google" => {
                let id_ = parts[1].parse::<i32>()?;
                let item: GoogleProblem = google_problem
                    .filter(rustgym_schema::schema::google_problem::dsl::id.eq(id_))
                    .first(&conn)?;
                let query_result = QueryResult::new(
                    parts[1].to_string(),
                    item.title.to_string(),
                    item.href(),
                    item.from(),
                );
                res.push(query_result);
            }
            _ => {}
        }
    }

    Ok(res)
}
