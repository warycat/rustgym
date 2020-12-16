use super::schema::leetcode_solution;
use rustgym_consts::*;
use std::fmt;

#[derive(Debug, Queryable, Insertable, new)]
#[table_name = "leetcode_solution"]
pub struct LeetcodeSolution {
    pub question_id: i32,
    filename: String,
    pub source: String,
}

impl fmt::Display for LeetcodeSolution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[Rust]({}/{})", LEETCODE_SRC, self.filename)
    }
}
