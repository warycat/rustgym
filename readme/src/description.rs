use super::*;

pub struct Description {
    pub id: u64,
    filename: String,
}

impl Description {
    fn new(id: u64, filename: String) -> Self {
        Description { id, filename }
    }
}

impl fmt::Display for Description {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]({}/{})", self.id, LEETCODE_DESC, self.filename)
    }
}

pub struct DescriptionList {
    pub descriptions: Vec<Description>,
}

impl DescriptionList {
    pub fn new(src_dir: std::path::PathBuf) -> Self {
        let mut descriptions: Vec<Description> = vec![];
        for entry in fs::read_dir(src_dir).unwrap() {
            let filename = entry.unwrap().file_name().to_str().unwrap().to_string();
            let n = filename.len();
            let id = filename[..n - 3].parse::<u64>().unwrap();
            let description = Description::new(id, filename);
            descriptions.push(description);
        }
        DescriptionList { descriptions }
    }
}
