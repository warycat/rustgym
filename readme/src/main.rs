mod description;
mod google;
mod leetcode;
mod solution;

#[macro_use]
extern crate derive_new;

use anyhow::Result;
use askama::Template;
use description::*;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use google::*;
use leetcode::*;
use rustgym_consts::*;
use rustgym_schema::AdventOfCodeDescription;
use rustgym_schema::GoogleProblem;
use rustgym_schema::LeetcodeQuestion;
use solution::*;
use std::fs;
use std::path::Path;

#[derive(Template, new)]
#[template(path = "readme.j2")]
struct ReadmeContext {
    leetcode_questions: Vec<LeetcodeQuestion>,
    adventofcode_descriptions: Vec<AdventOfCodeDescription>,
    google_problems: Vec<GoogleProblem>,
}

// type Tags = HashMap<i32, Vec<Tag>>;
// type Tag = (String, String);

fn main() -> Result<()> {
    use rustgym_schema::schema::adventofcode_description::dsl::*;
    use rustgym_schema::schema::adventofcode_solution::dsl::*;
    use rustgym_schema::schema::google_problem::dsl::*;
    use rustgym_schema::schema::leetcode_description::dsl::*;
    use rustgym_schema::schema::leetcode_question::dsl::*;
    use rustgym_schema::schema::leetcode_solution::dsl::*;

    let conn = SqliteConnection::establish(DATABASE_URL)?;

    let leetcode_json = LeetcodeData::new(
        LEETCODE_JSON_URL,
        // LEETCODE_TAG_URL
    );
    // let tags = leetcode_json.get_tags().unwrap_or_default();

    let leetcode_questions = leetcode_json.get_questions().unwrap_or_default();

    diesel::insert_into(leetcode_question)
        .values(&leetcode_questions)
        .execute(&conn)?;

    let src_dir = Path::new(LEETCODE_SRC);

    let leetcode_solutions = all_leetcode_solutions(&src_dir);
    diesel::insert_into(leetcode_solution)
        .values(&leetcode_solutions)
        .execute(&conn)?;

    let leetcode_desc_dir = Path::new(LEETCODE_DESC);
    let leetcode_descriptions = all_leetcode_descriptions(&leetcode_desc_dir);
    diesel::insert_into(leetcode_description)
        .values(&leetcode_descriptions)
        .execute(&conn)?;

    let adventofcode_desc_dir = Path::new(ADVENTOFCODE_DESC);
    let adventofcode_descriptions = all_adventofcode_descriptions(adventofcode_desc_dir);
    diesel::insert_into(adventofcode_description)
        .values(&adventofcode_descriptions)
        .execute(&conn)?;

    let adventofcode_src_dir = Path::new(ADVENTOFCODE_SRC);
    let adventofcode_solutions = all_adventofcode_solutions(adventofcode_src_dir);
    diesel::insert_into(adventofcode_solution)
        .values(&adventofcode_solutions)
        .execute(&conn)?;

    let google_src_dir = Path::new(GOOGLE_SRC);
    let google_problems: Vec<GoogleProblem> = all_google_problems(google_src_dir);
    diesel::insert_into(google_problem)
        .values(google_problems)
        .execute(&conn)?;

    let leetcode_questions: Vec<LeetcodeQuestion> = leetcode_question
        .order((
            rustgym_schema::schema::leetcode_question::dsl::level,
            rustgym_schema::schema::leetcode_question::dsl::id,
        ))
        .load(&conn)?;

    let adventofcode_descriptions = adventofcode_description
        .order((
            rustgym_schema::schema::adventofcode_description::year,
            rustgym_schema::schema::adventofcode_description::day,
        ))
        .load(&conn)?;

    let google_problems = google_problem.load(&conn)?;

    let readme_text: String = ReadmeContext::new(
        leetcode_questions,
        adventofcode_descriptions,
        google_problems,
    )
    .render()?;
    let readme_md = Path::new(README_MD);
    fs::write(&readme_md, readme_text).unwrap();
    Ok(())
}
