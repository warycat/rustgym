use std::collections::BTreeMap;
use std::collections::HashMap;
use std::env;
use std::fmt;
use std::fs;
use std::path::Path;
use util::*;

const TITLE: &str = "# Leetcode Solutions in Rust";
const CI: &str = "# leetcode_rs [![Build Status](https://travis-ci.org/warycat/leetcode_rs.svg?branch=master)](https://travis-ci.org/warycat/leetcode_rs)";
const LEETCODE_JSON_URL: &str = "https://leetcode.com/api/problems/algorithms/";
const LEETCODE_QUESTION_URL: &str = "https://leetcode.com/problems/";
const CARGO_MANIFEST_DIR: &str = "CARGO_MANIFEST_DIR";
const README_MD: &str = "README.md";
const SRC: &str = "src";

struct RustSolution {
    id: i64,
    filename: String,
}

impl RustSolution {
    fn new(id: i64, filename: String) -> Self {
        RustSolution { id, filename }
    }
}

impl fmt::Display for RustSolution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[solution](src/{})", self.filename)
    }
}

struct RustSolutionList {
    solutions: Vec<RustSolution>,
}

impl RustSolutionList {
    fn new(src_dir: std::path::PathBuf) -> Self {
        let mut solutions: Vec<RustSolution> = vec![];
        for entry in fs::read_dir(src_dir).unwrap() {
            let filename = entry.unwrap().file_name().to_str().unwrap().to_string();
            if let Some(0) = filename.find("_") {
                let s: Vec<String> = filename.split("_").map(|s| s.to_string()).collect();
                let id = s[1].clone().parse::<i64>().unwrap();
                let problem = RustSolution::new(id, filename);
                solutions.push(problem);
            }
        }
        solutions.sort_by_key(|x| x.id);
        RustSolutionList { solutions }
    }
}

struct LeetcodeJson {
    url: &'static str,
}

impl LeetcodeJson {
    fn new(url: &'static str) -> Self {
        LeetcodeJson { url }
    }

    fn get_text(&self) -> Result<String, Box<dyn std::error::Error>> {
        let resp = reqwest::blocking::get(self.url)?.text()?;
        Ok(format!("{}", resp))
    }

    fn get_questions(&self) -> Result<Vec<LeetcodeQuestion>, Box<dyn std::error::Error>> {
        let json_string = self.get_text()?;
        let value: serde_json::Value = serde_json::from_str(&json_string).unwrap();
        let pairs = value["stat_status_pairs"].as_array().unwrap();
        let mut questions = vec![];
        for pair in pairs {
            let stat = pair["stat"].as_object().unwrap();
            let id = stat["question_id"].as_i64().unwrap();
            let title = stat["question__title"].as_str().unwrap();
            let slug = stat["question__title_slug"].as_str().unwrap();
            questions.push(LeetcodeQuestion::new(id, title, slug))
        }
        Ok(questions)
    }
}

struct LeetcodeQuestion {
    id: i64,
    title: String,
    slug: String,
}

impl LeetcodeQuestion {
    fn new<S: Into<String>>(id: i64, title: S, slug: S) -> Self {
        LeetcodeQuestion {
            id,
            title: title.into(),
            slug: slug.into(),
        }
    }
}

impl fmt::Display for LeetcodeQuestion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}]({}{})",
            self.title, LEETCODE_QUESTION_URL, self.slug
        )
    }
}

struct LeetcodeQuestionList {
    questions: Vec<LeetcodeQuestion>,
}

impl LeetcodeQuestionList {
    fn new(questions: Vec<LeetcodeQuestion>) -> Self {
        LeetcodeQuestionList { questions }
    }
}

struct Readme {
    headers: Vec<String>,
    solution_list: RustSolutionList,
    question_list: LeetcodeQuestionList,
}

impl Readme {
    fn new(
        headers: Vec<String>,
        solution_list: RustSolutionList,
        question_list: LeetcodeQuestionList,
    ) -> Self {
        Readme {
            headers,
            solution_list,
            question_list,
        }
    }

    fn table(&self) -> String {
        let solutions = &self.solution_list.solutions;
        let questions = &self.question_list.questions;
        let m = solutions.len();
        let n = questions.len();
        let mut rows: Vec<(i64, String, String)> = vec![];
        let mut btm: BTreeMap<i64, String> = BTreeMap::new();
        let mut hm: HashMap<i64, String> = HashMap::new();
        for i in 0..n {
            let id = questions[i].id;
            btm.insert(id, questions[i].to_string());
        }
        for j in 0..m {
            let id = solutions[j].id;
            hm.insert(id, solutions[j].to_string());
        }
        for (id, question) in btm {
            let solution = hm.get(&id).unwrap_or(&"   ".to_string()).to_string();
            rows.push((id, question, solution));
        }
        let mut s = "|id|Question|Solution|\n".to_string();
        s += "|---|---|---|\n";
        for row in rows {
            s += &format!("|{}|{}|{}|\n", row.0, row.1, row.2);
        }
        s
    }
}

impl fmt::Display for Readme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = "".to_string();
        for header in &self.headers {
            s += &format!("{}\n\n", header);
        }
        s += &self.table();
        write!(f, "{}", s)
    }
}

fn main() {
    let leetcode_json = LeetcodeJson::new(LEETCODE_JSON_URL);
    let questions = leetcode_json.get_questions().unwrap_or_default();
    let question_list = LeetcodeQuestionList::new(questions);
    let cargo_dir = env::var_os(CARGO_MANIFEST_DIR).unwrap();
    let readme_md = Path::new(&cargo_dir).join(README_MD);
    let src_dir = Path::new(&cargo_dir).join(SRC);
    let solution_list = RustSolutionList::new(src_dir);
    let headers = vec_string![TITLE, CI];
    let readme = Readme::new(headers, solution_list, question_list);
    fs::write(&readme_md, format!("{}", readme)).unwrap();
}
