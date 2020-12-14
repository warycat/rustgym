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

const DATABASE_URL: &'static str = "rustgym.sqlite";

fn main() -> Result<()> {
    use rustgym_schema::schema::leetcode_question::dsl::*;
    let conn = SqliteConnection::establish(DATABASE_URL)?;
    let leetcode_json = LeetcodeData::new(LEETCODE_JSON_URL, LEETCODE_TAG_URL);
    let questions = leetcode_json.get_questions().unwrap_or_default();
    diesel::insert_into(leetcode_question)
        .values(&questions)
        .execute(&conn)?;
    let question_list = LeetcodeQuestionList::new(questions);
    let tags = leetcode_json.get_tags().unwrap_or_default();
    let cargo_dir = env::var_os(CARGO_MANIFEST_DIR).unwrap();
    let readme_md = Path::new(&cargo_dir).join("..").join(README_MD);
    let src_dir = Path::new(&cargo_dir).join("..").join(LEETCODE_SRC);
    let solution_list = RustSolutionList::new(src_dir);
    let desc_dir = Path::new(&cargo_dir).join("..").join(LEETCODE_DESC);
    let description_list = DescriptionList::new(desc_dir);
    let readme = Readme::new(
        TEMPLATE.to_string(),
        solution_list,
        question_list,
        description_list,
        tags,
    );
    fs::write(&readme_md, format!("{}", readme)).unwrap();
    Ok(())
}
