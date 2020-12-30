use super::*;
use fs::File;
use regex::Regex;
use rustgym_schema::AdventOfCodeSolution;
use rustgym_schema::LeetcodeSolution;
use std::io::Read;
use walkdir::WalkDir;

pub fn all_leetcode_solutions(src_dir: std::path::PathBuf) -> Vec<LeetcodeSolution> {
    let mut solutions: Vec<LeetcodeSolution> = vec![];
    for entry in WalkDir::new(src_dir) {
        let entry = entry.unwrap();
        if entry.file_type().is_file() {
            let filename = entry.file_name().to_str().unwrap().to_string();
            if let Some(0) = filename.find('_') {
                let s: Vec<String> = filename.split('_').map(|s| s.to_string()).collect();
                let id = s[1].clone().parse::<i32>().unwrap();
                let mut file = File::open(entry.path()).unwrap();
                let mut source = "".to_string();
                file.read_to_string(&mut source).unwrap();
                let problem = LeetcodeSolution::new(id, filename, source);
                solutions.push(problem);
            }
        }
    }
    solutions.sort_by_key(|x| x.question_id);
    solutions
}

pub fn all_adventofcode_solutions(src_dir: std::path::PathBuf) -> Vec<AdventOfCodeSolution> {
    let mut solutions: Vec<AdventOfCodeSolution> = vec![];
    let re_year_day = Regex::new(r"(\d+)/day(\d+)/mod.rs").unwrap();
    for entry in WalkDir::new(src_dir) {
        let entry = entry.unwrap();
        if entry.file_type().is_file() {
            let filename = entry.file_name().to_str().unwrap().to_string();
            let path = entry.path();
            let path_str = path.to_str().unwrap();
            if let Some(caps) = re_year_day.captures(path_str) {
                let year = caps[1].parse::<i32>().unwrap();
                let day = caps[2].parse::<i32>().unwrap();
                let mut file = File::open(path).unwrap();
                let mut source = "".to_string();
                file.read_to_string(&mut source).unwrap();
                let id = year * 100 + day;
                let solution = AdventOfCodeSolution::new(id, year, day, filename, source);
                solutions.push(solution);
            }
        }
    }
    solutions
}
