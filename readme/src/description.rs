use super::*;
use fs::File;
use regex::Regex;
use rustgym_schema::adventofcode_description::AdventOfCodeDescription;
use rustgym_schema::leetcode_description::LeetcodeDescription;
use std::io::Read;
use std::path::Path;
use walkdir::WalkDir;

pub fn all_leetcode_descriptions(src_dir: &Path) -> Vec<LeetcodeDescription> {
    let mut descriptions: Vec<LeetcodeDescription> = vec![];
    for entry in WalkDir::new(src_dir) {
        let entry = entry.unwrap();
        if entry.file_type().is_file() {
            let filename = entry.file_name().to_str().unwrap().to_string();
            let n = filename.len();
            let id = filename[..n - 3].parse::<i32>().unwrap();
            let mut file = File::open(entry.path()).unwrap();
            let mut html = "".to_string();
            file.read_to_string(&mut html).unwrap();
            let description = LeetcodeDescription::new(id, filename, html);
            descriptions.push(description);
        }
    }
    descriptions
}

pub fn all_adventofcode_descriptions(src_dir: &Path) -> Vec<AdventOfCodeDescription> {
    let mut descriptions: Vec<AdventOfCodeDescription> = vec![];
    let re_year_day = Regex::new(r"(\d+)/day(\d+).md").unwrap();
    let re_title = Regex::new(r": (.*) ---").unwrap();
    for entry in WalkDir::new(src_dir) {
        let entry = entry.unwrap();
        if entry.file_type().is_file() {
            let filename = entry.file_name().to_str().unwrap().to_string();
            let path = entry.path();
            let caps = re_year_day.captures(path.to_str().unwrap()).unwrap();
            let year = caps[1].parse::<i32>().unwrap();
            let day = caps[2].parse::<i32>().unwrap();
            let mut file = File::open(path).unwrap();
            let mut html = "".to_string();
            file.read_to_string(&mut html).unwrap();
            let caps = re_title.captures(&html).unwrap();
            let title = caps[1].to_string();
            let id = year * 100 + day;
            let description = AdventOfCodeDescription::new(id, year, day, title, filename, html);
            descriptions.push(description);
        }
    }
    descriptions
}
