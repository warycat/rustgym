use anyhow::Result;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use rustgym_consts::*;
use rustgym_schema::AdventOfCodeDescription;
use rustgym_schema::GoogleProblem;
use rustgym_schema::LeetcodeDescription;
use rustgym_schema::LeetcodeQuestion;
use sonic_channel::*;

fn cleanup(html: String) -> String {
    let text = html2text::from_read(html.as_bytes(), html.len());
    let text: String = text
        .chars()
        .map(|c| if c.is_ascii_alphabetic() { c } else { ' ' })
        .collect();
    text
}

fn main() -> Result<()> {
    use rustgym_schema::schema::adventofcode_description::dsl::*;
    use rustgym_schema::schema::google_problem::dsl::*;
    use rustgym_schema::schema::leetcode_description::dsl::*;
    use rustgym_schema::schema::leetcode_question::dsl::*;

    let channel = IngestChannel::start(SONIC_URL, SONIC_PASS)?;
    let conn = SqliteConnection::establish(DATABASE_URL)?;

    let dest = Dest::col_buc(SONIC_COLLECTION, SONIC_BUCKET);

    let leetcode_questions: Vec<LeetcodeQuestion> = leetcode_question.load(&conn)?;
    for item in leetcode_questions {
        let object = format!("leetcode_{}", item.id);
        let obj_dest = dest.clone().obj(&object);
        let text = cleanup(item.title);
        channel.push(PushRequest::new(obj_dest, &text))?;
    }

    let leetcode_descriptions: Vec<LeetcodeDescription> = leetcode_description.load(&conn)?;
    for item in leetcode_descriptions {
        let object = format!("leetcode_{}", item.id);
        let obj_dest = dest.clone().obj(&object);
        let text = cleanup(item.html);
        channel.push(PushRequest::new(obj_dest, &text))?;
    }

    let adventofcode_descriptions: Vec<AdventOfCodeDescription> =
        adventofcode_description.load(&conn)?;
    for item in adventofcode_descriptions {
        let object = format!("adventofcode_{}", item.id);
        let obj_dest = dest.clone().obj(object);
        let text = cleanup(item.title);
        channel.push(PushRequest::new(obj_dest.clone(), &text))?;
        let text = cleanup(item.html);
        channel.push(PushRequest::new(obj_dest, &text))?;
    }

    let google_problems: Vec<GoogleProblem> = google_problem.load(&conn)?;
    for item in google_problems {
        let object = format!("google_{}", item.id);
        let obj_dest = dest.clone().obj(object);
        let text = cleanup(item.title);
        channel.push(PushRequest::new(obj_dest.clone(), &text))?;
        let text = cleanup(item.problem);
        channel.push(PushRequest::new(obj_dest, &text))?;
    }
    Ok(())
}
