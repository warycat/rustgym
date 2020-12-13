use super::*;
use std::fmt;

#[derive(Debug)]
pub struct LeetcodeQuestion {
    pub id: u64,
    pub frontend_id: u64,
    pub title: String,
    pub slug: String,
    pub level: u64,
}

impl LeetcodeQuestion {
    pub fn new<S: Into<String>>(id: u64, title: S, slug: S, level: u64, frontend_id: u64) -> Self {
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

pub struct LeetcodeQuestionList {
    pub questions: Vec<LeetcodeQuestion>,
}

impl LeetcodeQuestionList {
    pub fn new(questions: Vec<LeetcodeQuestion>) -> Self {
        LeetcodeQuestionList { questions }
    }
}
