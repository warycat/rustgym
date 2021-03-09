use std::collections::HashMap;

#[derive(Default)]
struct FileSystem {
    paths: HashMap<String, i32>,
}

impl FileSystem {
    fn new() -> Self {
        FileSystem {
            paths: HashMap::new(),
        }
    }

    fn create_path(&mut self, path: String, value: i32) -> bool {
        let index = path.rfind('/').unwrap();
        let parent = &path[0..index];
        if !self.paths.contains_key(&path) && (parent.is_empty() || self.paths.contains_key(parent))
        {
            self.paths.insert(path, value);
            true
        } else {
            false
        }
    }

    fn get(&self, path: String) -> i32 {
        *self.paths.get(&path).unwrap_or(&-1)
    }
}

#[test]
fn test() {
    let mut fs = FileSystem::new();
    assert_eq!(fs.create_path("/leet".to_string(), 1), true);
    assert_eq!(fs.create_path("/leet/code".to_string(), 2), true);
    assert_eq!(fs.get("/leet/code".to_string()), 2);
    assert_eq!(fs.create_path("/c/d".to_string(), 2), false);
    assert_eq!(fs.get("/c".to_string()), -1);
}
