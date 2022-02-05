use crate::context::*;

#[derive(Queryable)]
pub struct LeetcodeIndexRow {
    pub id: i32,
    pub title: String,
    pub level: i32,
}

impl LeetcodeIndexRow {
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
}

#[derive(Template, new)]
#[template(path = "leetcode-index.j2")]
pub struct LeetcodeIndexContext {
    pub app: AppContext,
    pub session: SessionData,
    pub path: String,
    pub rows_easy: Vec<LeetcodeIndexRow>,
    pub rows_medium: Vec<LeetcodeIndexRow>,
    pub rows_hard: Vec<LeetcodeIndexRow>,
}

#[derive(Template, new)]
#[template(path = "leetcode-detail.j2")]
pub struct LeetcodeDetailContext {
    pub app: AppContext,
    pub session: SessionData,
    pub path: String,
    pub question: LeetcodeQuestion,
    pub description: LeetcodeDescription,
    pub solutions: Vec<LeetcodeSolution>,
}
