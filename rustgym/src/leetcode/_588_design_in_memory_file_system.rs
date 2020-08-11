use std::collections::BTreeMap;

#[derive(Debug)]
enum Entry {
    D(Dir),
    F(File),
}

#[derive(Debug)]
struct File {
    name: String,
    content: String,
}

impl File {
    fn new(name: String, content: String) -> Self {
        File { name, content }
    }

    fn name(&self) -> String {
        self.name.to_string()
    }

    fn append(&mut self, content: String) {
        self.content += &content;
    }

    fn content(&self) -> String {
        self.content.to_string()
    }
}

#[derive(Debug)]
struct Dir {
    name: String,
    entries: BTreeMap<String, Entry>,
}

impl Dir {
    fn new(name: String) -> Self {
        let entries = BTreeMap::new();
        Dir { name, entries }
    }

    fn list(&self) -> Vec<String> {
        self.entries.keys().map(|s| s.to_string()).collect()
    }
}

#[derive(Debug)]
struct FileSystem {
    root: Entry,
}

impl FileSystem {
    fn new() -> Self {
        let root = Entry::D(Dir::new("".to_string()));
        FileSystem { root }
    }

    fn ls(&self, path: String) -> Vec<String> {
        let mut e: &Entry = &self.root;
        for name in path.split('/').filter(|s| !s.is_empty()) {
            if let Entry::D(dir) = e {
                e = &dir.entries[name];
            } else {
                panic!();
            }
        }
        match e {
            Entry::D(d) => d.list(),
            Entry::F(f) => vec![f.name()],
        }
    }

    fn mkdir(&mut self, path: String) {
        let mut e: &mut Entry = &mut self.root;
        for name in path.split('/').filter(|s| !s.is_empty()) {
            if let Entry::D(dir) = e {
                e = dir
                    .entries
                    .entry(name.to_string())
                    .or_insert_with(|| Entry::D(Dir::new(name.to_string())))
            } else {
                panic!();
            }
        }
    }

    fn add_content_to_file(&mut self, path: String, content: String) {
        let mut e: &mut Entry = &mut self.root;
        for name in path.split('/').filter(|s| !s.is_empty()) {
            if let Entry::D(dir) = e {
                e = dir
                    .entries
                    .entry(name.to_string())
                    .or_insert_with(|| Entry::F(File::new(name.to_string(), "".to_string())))
            } else {
                panic!();
            }
        }
        if let Entry::F(file) = e {
            file.append(content);
        } else {
            panic!();
        }
    }

    fn read_content_from_file(&mut self, path: String) -> String {
        let mut e: &mut Entry = &mut self.root;
        for name in path.split('/').filter(|s| !s.is_empty()) {
            if let Entry::D(dir) = e {
                e = dir.entries.get_mut(name).unwrap();
            } else {
                panic!();
            }
        }
        if let Entry::F(file) = e {
            file.content()
        } else {
            panic!()
        }
    }
}

#[test]
fn test() {
    let mut fs = FileSystem::new();
    assert_eq!(fs.ls("/".to_string()), vec_string![]);
    fs.mkdir("/a/b/c".to_string());
    fs.add_content_to_file("/a/b/c/d".to_string(), "hello".to_string());
    assert_eq!(fs.ls("/".to_string()), vec_string!["a"]);
    assert_eq!(
        fs.read_content_from_file("/a/b/c/d".to_string()),
        "hello".to_string()
    );

    let mut fs = FileSystem::new();
    fs.mkdir("/goowmfn".to_string());
    assert_eq!(fs.ls("/goowmfn".to_string()), vec_string![]);
    assert_eq!(fs.ls("/".to_string()), vec_string!["goowmfn"]);
    fs.mkdir("/z".to_string());
    assert_eq!(fs.ls("/".to_string()), vec_string!["goowmfn", "z"]);
    assert_eq!(fs.ls("/".to_string()), vec_string!["goowmfn", "z"]);
    fs.add_content_to_file("/goowmfn/c".to_string(), "shetopcy".to_string());
    assert_eq!(fs.ls("/z".to_string()), vec_string![]);
    assert_eq!(fs.ls("/goowmfn/c".to_string()), vec_string!["c"]);
}
