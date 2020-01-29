struct Solution;

use std::collections::BTreeMap;

#[derive(Debug, PartialEq, Eq, Clone, Default)]
struct Trie {
    children: BTreeMap<char, Trie>,
    end: bool,
}

impl Trie {
    fn new() -> Self {
        let children: BTreeMap<char, Trie> = BTreeMap::new();
        Trie {
            children,
            end: false,
        }
    }
    fn from_words(words: Vec<String>) -> Self {
        let mut trie = Trie::new();
        for word in words {
            trie.insert(word);
        }
        trie
    }
    fn insert(&mut self, s: String) {
        let mut link = self;
        for c in s.chars() {
            link = link.children.entry(c).or_default();
        }
        link.end = true;
    }
    fn dfs(&self, s: &mut String, max: &mut String) {
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
    fn longest_word(words: Vec<String>) -> String {
        let trie = Trie::from_words(words);
        let mut s: String = "".to_string();
        let mut max: String = "".to_string();
        trie.dfs(&mut s, &mut max);
        max
    }
}

#[test]
fn test() {
    let words: Vec<String> = ["a", "banana", "app", "appl", "ap", "apply", "apple"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    let res = "apple".to_string();
    assert_eq!(Solution::longest_word(words), res);
}
