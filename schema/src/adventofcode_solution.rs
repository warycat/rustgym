use super::schema::adventofcode_solution;

#[derive(Debug, Queryable, Insertable, new)]
#[table_name = "adventofcode_solution"]
pub struct AdventOfCodeSolution {
    pub id: i32,
    pub year: i32,
    pub day: i32,
    filename: String,
    pub source: String,
}
