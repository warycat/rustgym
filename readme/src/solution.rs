use super::*;
use fs::File;
use rustgym_schema::LeetcodeSolution;
use std::io::Read;

pub struct RustSolutionList {
    pub solutions: Vec<LeetcodeSolution>,
}

impl RustSolutionList {
    pub fn new(src_dir: std::path::PathBuf) -> Self {
        let mut solutions: Vec<LeetcodeSolution> = vec![];
        for entry in fs::read_dir(src_dir).unwrap() {
            let dir = entry.unwrap();
            let filename = dir.file_name().to_str().unwrap().to_string();
            if let Some(0) = filename.find('_') {
                let s: Vec<String> = filename.split('_').map(|s| s.to_string()).collect();
                let id = s[1].clone().parse::<i32>().unwrap();
                let mut file = File::open(dir.path()).unwrap();
                let mut source = "".to_string();
                file.read_to_string(&mut source).unwrap();
                let problem = LeetcodeSolution::new(id, filename, source);
                solutions.push(problem);
            }
        }
        solutions.sort_by_key(|x| x.question_id);
        RustSolutionList { solutions }
    }
}
