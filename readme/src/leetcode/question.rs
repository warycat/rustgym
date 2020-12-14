use rustgym_schema::LeetcodeQuestion;

pub struct LeetcodeQuestionList {
    pub questions: Vec<LeetcodeQuestion>,
}

impl LeetcodeQuestionList {
    pub fn new(questions: Vec<LeetcodeQuestion>) -> Self {
        LeetcodeQuestionList { questions }
    }
}
