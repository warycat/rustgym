use crate::schema::nes_rom;
use rustgym_consts::*;
use std::fmt;

#[derive(Debug, Queryable, Insertable, new)]
#[table_name = "nes_rom"]
pub struct NesRom {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub filename: String,
    pub image: String,
    pub size: i32,
    pub md5: String,
}

impl fmt::Display for NesRom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[Rust]({}/{})", LEETCODE_SRC, self.filename)
    }
}
