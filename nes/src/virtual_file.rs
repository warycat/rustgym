#[derive(Default)]
pub struct VirtualFile {
    pub filename: String,
    pub data: Vec<u8>,
}

impl VirtualFile {
    pub fn new(filename: &str, data: &[u8]) -> Self {
        let data = data.to_vec();
        let filename = filename.to_string();
        VirtualFile { filename, data }
    }
    pub fn is_valid(&self) -> bool {
        self.data.len() > 0
    }
}
