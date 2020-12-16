use super::schema::leetcode_solution;
use rustgym_consts::*;
use std::fmt;

#[derive(Debug, Queryable, Insertable)]
#[table_name = "leetcode_solution"]
pub struct LeetcodeSolution {
    pub id: i32,
    filename: String,
    pub source: String,
}

impl LeetcodeSolution {
    pub fn new(id: i32, filename: String, source: String) -> Self {
        LeetcodeSolution {
            id,
            filename,
            source,
        }
    }
}

impl fmt::Display for LeetcodeSolution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[Rust]({}/{})", LEETCODE_SRC, self.filename)
    }
}
