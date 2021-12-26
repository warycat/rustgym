use crate::context::*;

#[derive(Queryable)]
pub struct NesIndexRow {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub image: String,
}

#[derive(Template, new)]
#[template(path = "nes-index.j2")]
pub struct NesIndexContext {
    pub app: AppContext,
    pub session: SessionData,
    pub path: String,
    pub rows: Vec<NesIndexRow>,
}

impl NesIndexRow {
    pub fn href(&self) -> String {
        format!("/nes/{}", self.id)
    }
}

#[derive(Template, new)]
#[template(path = "nes-detail.j2")]
pub struct NesDetailContext {
    pub app: AppContext,
    pub session: SessionData,
    pub path: String,
    pub rom: NesRom,
}
