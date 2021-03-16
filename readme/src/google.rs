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
            match filename.as_str() {
                "analysis.md" => {
                    let google_problem = GoogleProblem::with_path(&path);
                    hm.entry(google_problem.id)
                        .or_insert(google_problem)
                        .analysis = read_to_string(path).unwrap();
                }
                "problem.md" => {
                    let google_problem = GoogleProblem::with_path(&path);
                    hm.entry(google_problem.id)
                        .or_insert(google_problem)
                        .problem = read_to_string(path).unwrap();
                }
                "mod.rs" => {
                    let google_problem = GoogleProblem::with_path(&path);
                    hm.entry(google_problem.id)
                        .or_insert(google_problem)
                        .solution = read_to_string(path).unwrap();
                }
                _ => {}
            }
        }
    }
    hm.into_iter().map(|(_, v)| v).collect()
}
