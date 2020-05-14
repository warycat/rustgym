use std::collections::HashMap;

#[derive(PartialEq, Eq, Clone, Debug, Default)]
pub struct Trie {
    children: HashMap<char, Trie>,
    end: bool,
}

impl Trie {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, word: String) {
        let mut link = self;
        for c in word.chars() {
            link = link.children.entry(c).or_default();
        }
        link.end = true;
    }

    pub fn search(&self, word: String) -> bool {
        let mut link = self;
        for c in word.chars() {
            if let Some(child) = link.children.get(&c) {
                link = child;
            } else {
                return false;
            }
        }
        link.end
    }
    pub fn starts_with(&self, word: String) -> bool {
        let mut link = self;
        for c in word.chars() {
            if let Some(child) = link.children.get(&c) {
                link = child;
            } else {
                return false;
            }
        }
        true
    }
}

#[test]
fn test() {
    let mut trie = Trie::new();
    let word = "apple".to_string();
    trie.insert(word);
    let word = "apple".to_string();
    assert_eq!(trie.search(word), true);
    let word = "app".to_string();
    assert_eq!(trie.search(word), false);
    let word = "app".to_string();
    assert_eq!(trie.starts_with(word), true);
    let word = "app".to_string();
    trie.insert(word);
    let word = "app".to_string();
    assert_eq!(trie.search(word), true);
}
