use super::schema::adventofcode_description;

#[derive(Debug, Queryable, Insertable, new)]
#[table_name = "adventofcode_description"]
pub struct AdventOfCodeDescription {
    pub id: i32,
    pub year: i32,
    pub day: i32,
    pub title: String,
    filename: String,
    pub html: String,
}

impl AdventOfCodeDescription {
    pub fn href(&self) -> String {
        format!("/adventofcode/{}", self.id)
    }

    pub fn from(&self) -> String {
        format!("AdventOfCode {}", self.year)
    }
}
