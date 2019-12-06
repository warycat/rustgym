struct Solution;

use std::collections::BTreeMap;

#[derive(Debug, PartialEq, Eq, Clone, Default)]
struct TrieNode {
    children: BTreeMap<char, TrieNode>,
    end: bool,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            children: BTreeMap::new(),
            end: false,
        }
    }
}

#[derive(Debug)]
struct Trie {
    root: TrieNode,
}

impl Trie {
    fn new() -> Self {
        Trie {
            root: TrieNode::new(),
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
        let mut link = &mut self.root;
        for c in s.chars() {
            link = link.children.entry(c).or_default();
        }
        link.end = true;
    }
    fn dfs(&self, link: &TrieNode, s: &mut String, max: &mut String) {
        if link.end {
            if s.len() > max.len() {
                *max = s.clone();
            }
        }
        if link.end || link == &self.root {
            for (&c, child) in link.children.iter() {
                s.push(c);
                self.dfs(child, s, max);
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
        trie.dfs(&trie.root, &mut s, &mut max);
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
