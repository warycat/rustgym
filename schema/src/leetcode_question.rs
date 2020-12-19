use super::schema::leetcode_question;
use rustgym_consts::*;
use std::fmt;

#[derive(Debug, Queryable, Insertable, new)]
#[table_name = "leetcode_question"]
pub struct LeetcodeQuestion {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub level: i32,
}

impl fmt::Display for LeetcodeQuestion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}]({}{})",
            self.title, LEETCODE_QUESTION_URL, self.slug
        )
    }
}
