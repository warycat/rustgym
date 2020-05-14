use std::collections::HashMap;

#[derive(PartialEq, Eq, Default, Debug)]
pub struct Trie {
    pub children: HashMap<char, Trie>,
    pub end: bool,
}

impl Trie {
    pub fn insert(&mut self, s: &str) {
        let mut link = self;
        for c in s.chars() {
            link = link.children.entry(c).or_default();
        }
        link.end = true;
    }

    pub fn search(&self, s: &str) -> bool {
        if s.is_empty() {
            return self.end;
        }
        let c = s.chars().next().unwrap();
        if c == '.' {
            for child in self.children.values() {
                if Self::search(child, &s[1..]) {
                    return true;
                }
            }
        } else {
            if let Some(child) = self.children.get(&c) {
                return Self::search(&child, &s[1..]);
            } else {
                return false;
            }
        }
        false
    }
}

#[derive(Default)]
pub struct WordDictionary {
    pub trie: Trie,
}

impl WordDictionary {
    pub fn new() -> Self {
        WordDictionary {
            trie: Trie::default(),
        }
    }
    pub fn add_word(&mut self, word: String) {
        self.trie.insert(&word);
    }
    pub fn search(&self, word: String) -> bool {
        self.trie.search(&word)
    }
}

#[test]
fn test() {
    let mut wd = WordDictionary::new();
    wd.add_word("bad".to_string());
    wd.add_word("dad".to_string());
    wd.add_word("mad".to_string());
    assert_eq!(wd.search("pad".to_string()), false);
    assert_eq!(wd.search("bad".to_string()), true);
    assert_eq!(wd.search(".ad".to_string()), true);
    assert_eq!(wd.search("b..".to_string()), true);
}
