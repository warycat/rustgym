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

impl LeetcodeQuestion {
    pub fn level_str(&self) -> &str {
        match self.level {
            1 => "Easy",
            2 => "Medium",
            3 => "Hard",
            _ => "",
        }
    }

    pub fn href(&self) -> String {
        format!("/leetcode/{}", self.id)
    }

    pub fn from(&self) -> String {
        format!("Leetcode {}", self.level_str())
    }

    pub fn practice_url(&self) -> String {
        format!("https://leetcode.com/problems/{}/", self.slug)
    }
}
