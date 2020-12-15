use super::*;
use fs::File;
use rustgym_schema::leetcode_description::LeetcodeDescription;
use std::io::Read;

pub struct DescriptionList {
    pub descriptions: Vec<LeetcodeDescription>,
}

impl DescriptionList {
    pub fn new(src_dir: std::path::PathBuf) -> Self {
        let mut descriptions: Vec<LeetcodeDescription> = vec![];
        for entry in fs::read_dir(src_dir).unwrap() {
            let dir = entry.unwrap();
            let filename = dir.file_name().to_str().unwrap().to_string();
            let n = filename.len();
            let id = filename[..n - 3].parse::<i32>().unwrap();
            let mut file = File::open(dir.path()).unwrap();
            let mut html = "".to_string();
            file.read_to_string(&mut html).unwrap();
            let description = LeetcodeDescription::new(id, filename, html);
            descriptions.push(description);
        }
        DescriptionList { descriptions }
    }
}
