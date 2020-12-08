use super::*;

pub struct RustSolution {
    pub id: u64,
    filename: String,
}

impl RustSolution {
    fn new(id: u64, filename: String) -> Self {
        RustSolution { id, filename }
    }
}

impl fmt::Display for RustSolution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[Rust]({}/{})", LEETCODE_SRC, self.filename)
    }
}

pub struct RustSolutionList {
    pub solutions: Vec<RustSolution>,
}

impl RustSolutionList {
    pub fn new(src_dir: std::path::PathBuf) -> Self {
        let mut solutions: Vec<RustSolution> = vec![];
        for entry in fs::read_dir(src_dir).unwrap() {
            let filename = entry.unwrap().file_name().to_str().unwrap().to_string();
            if let Some(0) = filename.find('_') {
                let s: Vec<String> = filename.split('_').map(|s| s.to_string()).collect();
                let id = s[1].clone().parse::<u64>().unwrap();
                let problem = RustSolution::new(id, filename);
                solutions.push(problem);
            }
        }
        solutions.sort_by_key(|x| x.id);
        RustSolutionList { solutions }
    }
}
