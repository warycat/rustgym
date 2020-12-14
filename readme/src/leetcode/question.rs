use super::*;
use rustgym_schema::LeetcodeQuestion;
use std::fmt;

pub struct LeetcodeQuestionList {
    pub questions: Vec<LeetcodeQuestion>,
}

impl LeetcodeQuestionList {
    pub fn new(questions: Vec<LeetcodeQuestion>) -> Self {
        LeetcodeQuestionList { questions }
    }
}
