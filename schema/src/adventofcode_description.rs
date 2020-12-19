use super::schema::adventofcode_description;
use rustgym_consts::*;
use std::fmt;

#[derive(Debug, Queryable, Insertable, new)]
#[table_name = "adventofcode_description"]
pub struct AdventOfCodeDescription {
    pub id: i32,
    pub year: i32,
    pub day: i32,
    pub title: String,
    filename: String,
    pub html: String,
}
