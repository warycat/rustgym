struct Solution;
use std::collections::HashMap;

#[derive(Default)]
struct Trie {
    children: HashMap<char, Trie>,
    end: bool,
}

impl Trie {
    fn new() -> Self {
        Trie::default()
    }

    fn insert(&mut self, s: String) {
        let mut link = self;
        for c in s.chars() {
            link = link.children.entry(c).or_default();
        }
        link.end = true;
    }

    fn map<'a>(&self, s: &'a str) -> &'a str {
        let mut link = self;
        for (size, c) in s.char_indices() {
            if link.end {
                return &s[0..size];
            }
            if let Some(next) = link.children.get(&c) {
                link = next;
            } else {
                break;
            }
        }
        s
    }
}

impl Solution {
    fn replace_words(dict: Vec<String>, sentence: String) -> String {
        let mut trie = Trie::new();
        for word in dict {
            trie.insert(word);
        }
        sentence
            .split_whitespace()
            .map(|s| trie.map(s))
            .collect::<Vec<&str>>()
            .join(" ")
    }
}

#[test]
fn test() {
    let dict = vec_string!["cat", "bat", "rat"];
    let sentence = "the cattle was rattled by the battery".to_string();
    let res = "the cat was rat by the bat".to_string();
    assert_eq!(Solution::replace_words(dict, sentence), res);
}
