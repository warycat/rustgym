use super::schema::leetcode_description;
use rustgym_consts::*;
use std::fmt;

#[derive(Debug, Queryable, Insertable)]
#[table_name = "leetcode_description"]
pub struct LeetcodeDescription {
    pub id: i32,
    filename: String,
    pub html: String,
}

impl LeetcodeDescription {
    pub fn new(id: i32, filename: String, html: String) -> Self {
        LeetcodeDescription { id, filename, html }
    }
}

impl fmt::Display for LeetcodeDescription {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]({}/{})", self.id, LEETCODE_DESC, self.filename)
    }
}
