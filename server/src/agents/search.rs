use crate::agents::envelope::Envelope;
use crate::agents::package::Package;
use crate::db::*;
use actix::prelude::*;
use anyhow::Result;
use diesel::prelude::*;
use log::info;
use rustgym_consts::*;
use rustgym_msg::Msg;
use rustgym_msg::QueryResult;
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

impl Handler<Package> for SearchAgent {
    type Result = ();
    fn handle(&mut self, package: Package, _ctx: &mut Context<Self>) {
        let Package {
            client_addr,
            envelope,
        } = package;
        let Envelope { client_info, msg } = envelope;

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
                let text: String = cleanup(text);
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
                            info!("reconnect {:?}", err);
                            self.reconnect();
                        }
                    }
                    let msg = Msg::SearchSuggestions(suggestions);
                    let envelope = Envelope::new(client_info, msg);
                    let package = Package::new(client_addr.clone(), envelope);
                    client_addr.do_send(package);
                }
            }
            QueryText(text) => {
                let text: String = cleanup(text);
                let mut query_results: Vec<QueryResult> = vec![];
                match self
                    .search_channel
                    .query(SONIC_COLLECTION, SONIC_BUCKET, &text)
                {
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
                let msg = Msg::QueryResults(query_results);
                let envelope = Envelope::new(client_info, msg);
                let package = Package::new(client_addr.clone(), envelope);
                client_addr.do_send(package);
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
