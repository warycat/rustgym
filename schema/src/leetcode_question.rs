use super::schema::leetcode_question;
use rustgym_consts::*;
use std::fmt;

#[derive(Debug, Queryable, Insertable)]
#[table_name = "leetcode_question"]
pub struct LeetcodeQuestion {
    pub id: i32,
    pub frontend_id: i32,
    pub title: String,
    pub slug: String,
    pub level: i32,
}

impl LeetcodeQuestion {
    pub fn new<S: Into<String>>(id: i32, title: S, slug: S, level: i32, frontend_id: i32) -> Self {
        LeetcodeQuestion {
            id,
            title: title.into(),
            slug: slug.into(),
            level,
            frontend_id,
        }
    }
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
