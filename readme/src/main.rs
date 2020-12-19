mod description;
mod leetcode;
mod readme;
mod solution;

use anyhow::Result;
use description::*;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use leetcode::*;
use readme::*;
use rustgym_consts::*;
use solution::*;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::env;
use std::fmt;
use std::fs;
use std::path::Path;

const TEMPLATE: &str = include_str!("template.md");

type Tags = HashMap<i32, Vec<Tag>>;
type Tag = (String, String);

fn main() -> Result<()> {
    use rustgym_schema::schema::adventofcode_description::dsl::*;
    use rustgym_schema::schema::adventofcode_solution::dsl::*;
    use rustgym_schema::schema::leetcode_description::dsl::*;
    use rustgym_schema::schema::leetcode_question::dsl::*;
    use rustgym_schema::schema::leetcode_solution::dsl::*;

    let conn = SqliteConnection::establish(DATABASE_URL)?;
    let cargo_dir = env::var_os(CARGO_MANIFEST_DIR).unwrap();

    let leetcode_json = LeetcodeData::new(LEETCODE_JSON_URL, LEETCODE_TAG_URL);
    let tags = leetcode_json.get_tags().unwrap_or_default();

    let leetcode_questions = leetcode_json.get_questions().unwrap_or_default();
    diesel::insert_into(leetcode_question)
        .values(&leetcode_questions)
        .execute(&conn)?;

    let src_dir = Path::new(&cargo_dir).join("..").join(LEETCODE_SRC);

    let leetcode_solutions = all_leetcode_solutions(src_dir);
    diesel::insert_into(leetcode_solution)
        .values(&leetcode_solutions)
        .execute(&conn)?;

    let leetcode_desc_dir = Path::new(&cargo_dir).join("..").join(LEETCODE_DESC);
    let leetcode_descriptions = all_leetcode_descriptions(leetcode_desc_dir);
    diesel::insert_into(leetcode_description)
        .values(&leetcode_descriptions)
        .execute(&conn)?;

    let adventofcode_desc_dir = Path::new(&cargo_dir).join("..").join(ADVENTOFCODE_DESC);
    let adventofcode_descriptions = all_adventofcode_descriptions(adventofcode_desc_dir);
    diesel::insert_into(adventofcode_description)
        .values(&adventofcode_descriptions)
        .execute(&conn)?;

    let adventofcode_src_dir = Path::new(&cargo_dir).join("..").join(ADVENTOFCODE_SRC);
    let adventofcode_solutions = all_adventofcode_solutions(adventofcode_src_dir);
    diesel::insert_into(adventofcode_solution)
        .values(&adventofcode_solutions)
        .execute(&conn)?;

    let readme = Readme::new(
        TEMPLATE.to_string(),
        leetcode_solutions,
        leetcode_questions,
        leetcode_descriptions,
        tags,
    );
    let readme_md = Path::new(&cargo_dir).join("..").join(README_MD);
    fs::write(&readme_md, format!("{}", readme)).unwrap();
    Ok(())
}
