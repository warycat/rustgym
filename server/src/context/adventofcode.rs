use crate::context::*;

#[derive(Queryable)]
pub struct AdventOfCodeIndexRow {
    pub id: i32,
    pub year: i32,
    pub day: i32,
    pub title: String,
}

#[derive(Template, new)]
#[template(path = "adventofcode-index.j2")]
pub struct AdventOfCodeIndexContext {
    pub app: AppContext,
    pub session: SessionData,
    pub path: String,
    pub rows: Vec<AdventOfCodeIndexRow>,
}

impl AdventOfCodeIndexRow {
    pub fn href(&self) -> String {
        format!("/adventofcode/{}", self.id)
    }
}

#[derive(Template, new)]
#[template(path = "adventofcode-detail.j2")]
pub struct AdventOfCodeDetailContext {
    pub app: AppContext,
    pub session: SessionData,
    pub path: String,
    pub description: AdventOfCodeDescription,
    pub solution: AdventOfCodeSolution,
}
