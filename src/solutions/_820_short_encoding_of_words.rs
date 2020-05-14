struct Solution;

use std::collections::HashMap;

#[derive(PartialEq, Eq, Default)]
struct Trie {
    children: HashMap<char, Trie>,
    end: bool,
}

impl Trie {
    fn new() -> Self {
        Self::default()
    }
    fn insert<I>(&mut self, iter: I)
    where
        I: Iterator<Item = char>,
    {
        let mut link = self;
        for c in iter {
            link = link.children.entry(c).or_default();
        }
        link.end = true;
    }

    fn dfs(&self, length: usize, sum: &mut usize) {
        let mut is_leaf = true;
        for link in self.children.values() {
            is_leaf = false;
            link.dfs(length + 1, sum);
        }
        if is_leaf {
            *sum += length + 1;
        }
    }
}

impl Solution {
    fn minimum_length_encoding(words: Vec<String>) -> i32 {
        let mut trie = Trie::new();
        for word in words {
            trie.insert(word.chars().rev());
        }
        let mut res = 0;
        trie.dfs(0, &mut res);
        res as i32
    }
}

#[test]
fn test() {
    let words = vec_string!["time", "me", "bell"];
    let res = 10;
    assert_eq!(Solution::minimum_length_encoding(words), res);
}
