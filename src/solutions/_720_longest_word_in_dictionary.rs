pub struct Solution;

use std::collections::BTreeMap;

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct Trie {
    children: BTreeMap<char, Trie>,
    end: bool,
}

impl Trie {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn from_words(words: Vec<String>) -> Self {
        let mut trie = Trie::new();
        for word in words {
            trie.insert(word);
        }
        trie
    }
    pub fn insert(&mut self, s: String) {
        let mut link = self;
        for c in s.chars() {
            link = link.children.entry(c).or_default();
        }
        link.end = true;
    }
    pub fn dfs(&self, s: &mut String, max: &mut String) {
        if self.end {
            if s.len() > max.len() {
                *max = s.clone();
            }
        }
        if s.is_empty() || self.end {
            for (&c, child) in self.children.iter() {
                s.push(c);
                child.dfs(s, max);
                s.pop();
            }
        }
    }
}

impl Solution {
    pub fn longest_word(words: Vec<String>) -> String {
        let trie = Trie::from_words(words);
        let mut s: String = "".to_string();
        let mut max: String = "".to_string();
        trie.dfs(&mut s, &mut max);
        max
    }
}

#[test]
fn test() {
    let words: Vec<String> = vec_string!["a", "banana", "app", "appl", "ap", "apply", "apple"];
    let res = "apple".to_string();
    assert_eq!(Solution::longest_word(words), res);
}
