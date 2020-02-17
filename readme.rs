use std::env;
use std::fs;
use std::path::Path;

struct Problem {
    number: i32,
    name: String,
    filename: String,
}

impl Problem {
    fn new(number: i32, name: String, filename: String) -> Self {
        Problem {
            number,
            name,
            filename,
        }
    }

    fn md_link(&self) -> String {
        format!("[{}. {}](src/{})\n", self.number, self.name, self.filename)
    }
}

fn main(){
    let cargo_dir = env::var_os("CARGO_MANIFEST_DIR").unwrap();
    let readme_md = Path::new(&cargo_dir).join("README.md");
    let src_dir = Path::new(&cargo_dir).join("src");
    println!("{:?}", readme_md);
    let mut readme_content = "".to_string();
    let title = "# Leetcode Solutions in Rust\n";
    let ci = "# leetcode_rs [![Build Status](https://travis-ci.org/warycat/leetcode_rs.svg?branch=master)](https://travis-ci.org/warycat/leetcode_rs)\n";
    readme_content += title;
    readme_content += ci;
    let mut problems: Vec<Problem> = vec![];
    for entry in fs::read_dir(src_dir).unwrap() {
        let filename = entry.unwrap().file_name().to_str().unwrap().to_string();
        if let Some(0) = filename.find("_"){
            let ss: Vec<String> = filename.split(".").map(|s| s.to_string()).collect();
            let s: Vec<String> = ss[0].split("_").map(|s| s.to_string()).collect();
            let number = s[1].clone().parse::<i32>().unwrap();
            let name:String = s[2..].join(" ");
            let problem = Problem::new(number, name, filename);
            problems.push(problem);
        }
    }
    problems.sort_by_key(|x| x.number);
    for p in problems {
        readme_content += &p.md_link();
    }
    fs::write(&readme_md, readme_content).unwrap();
}
