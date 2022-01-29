use log::Level;
use log::LevelFilter;
use std::time::Duration;

pub const LEETCODE_ALGORITHMS_URL: &str = "https://leetcode.com/api/problems/algorithms/";
pub const LEETCODE_CONCURRENCY_URL: &str = "https://leetcode.com/api/problems/concurrency/";
pub const LEETCODE_TAG_URL: &str = "https://leetcode.com/problems/api/tags/";
pub const LEETCODE_QUESTION_URL: &str = "https://leetcode.com/problems/";
pub const CARGO_MANIFEST_DIR: &str = "CARGO_MANIFEST_DIR";
pub const README_MD: &str = "README.md";
pub const LEETCODE_SRC: &str = "leetcode/src/";
pub const LEETCODE_DESC: &str = "leetcode/desc/";
pub const ADVENTOFCODE_SRC: &str = "adventofcode/src/";
pub const ADVENTOFCODE_DESC: &str = "adventofcode/desc/";
pub const GOOGLE_SRC: &str = "google/src/";
// pub const HACKERRANK_SRC: &str = "rustgym/src/hackerrank";
// pub const HACKERRANK_DESC: &str = "rustgym/desc/hackerrank";

pub const DATABASE_URL: &str = "./data/sqlite/rustgym.sqlite";
pub const OPENVGDB_URL: &str = "./data/sqlite/openvgdb.sqlite";
pub const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
pub const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);
pub const SONIC_URL: &str = "localhost:1491";
pub const SONIC_PASS: &str = "SecretPassword";
pub const SONIC_COLLECTION: &str = "collection";
pub const SONIC_BUCKET: &str = "bucket";

pub const SEARCH_PLACEHOLDER: &str = "Search Rust Solutions";

pub const TIME_SLICE: i32 = 100;
pub const MIME_TYPE: &str = "video/webm;codecs=vp9,opus";
pub const STREAM_DIR: &str = "./stream";
pub const DATA_DIR: &str = "./data";

pub const LOG_LEVEL: Level = Level::Info;
pub const LOG_LEVEL_FILTER: LevelFilter = LevelFilter::Info;
