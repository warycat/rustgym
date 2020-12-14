#[macro_use]
extern crate diesel;

#[macro_use]
extern crate serde;

pub mod leetcode_question;
pub mod schema;

pub use leetcode_question::*;
