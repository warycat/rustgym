use rustgym_schema::GoogleProblem;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::Path;
use walkdir::WalkDir;

pub fn all_google_problems(path: &Path) -> Vec<GoogleProblem> {
    let mut hm: HashMap<i32, GoogleProblem> = HashMap::new();
    for entry in WalkDir::new(path) {
        let entry = entry.unwrap();
        if entry.file_type().is_file() {
            let filename = entry.file_name().to_str().unwrap().to_string();
            let path: &Path = entry.path();
            if let Some(google_problem) = GoogleProblem::with_path(path) {
                match filename.as_str() {
                    "analysis.md" => {
                        hm.entry(google_problem.id)
                            .or_insert(google_problem)
                            .analysis = read_file(path);
                    }
                    "problem.md" => {
                        hm.entry(google_problem.id)
                            .or_insert(google_problem)
                            .problem = read_file(path);
                    }
                    "mod.rs" => {
                        hm.entry(google_problem.id)
                            .or_insert(google_problem)
                            .solution = read_file(path);
                    }
                    "input.txt" => {
                        hm.entry(google_problem.id).or_insert(google_problem).input =
                            read_file(path);
                    }
                    "output.txt" => {
                        hm.entry(google_problem.id).or_insert(google_problem).output =
                            read_file(path);
                    }
                    _ => {}
                }
            }
        }
    }
    hm.into_iter().map(|(_, v)| v).collect()
}

fn read_file(path: &Path) -> String {
    let s = read_to_string(path).unwrap();
    let mut res = "".to_string();
    for c in s.chars() {
        match c {
            '≤' => {
                res.push('<');
                res.push('=');
            }
            '≥' => {
                res.push('>');
                res.push('=');
            }
            _ => {
                res.push(c);
            }
        }
    }
    res
}
