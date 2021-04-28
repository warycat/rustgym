use std::collections::HashMap;

#[derive(Default)]
struct Trie {
    children: HashMap<char, Trie>,
    count: i32,
}

impl Trie {
    fn new() -> Self {
        Trie::default()
    }

    fn insert(&mut self, word: String) {
        let mut link = self;
        for c in word.chars() {
            link = link.children.entry(c).or_default();
        }
        link.count += 1;
    }

    fn count_words_equal_to(&self, word: String) -> i32 {
        let mut link = self;
        for c in word.chars() {
            if let Some(next) = link.children.get(&c) {
                link = next;
            } else {
                return 0;
            }
        }
        link.count
    }

    fn count_words_starting_with(&self, prefix: String) -> i32 {
        let mut link = self;
        for c in prefix.chars() {
            if let Some(next) = link.children.get(&c) {
                link = next;
            } else {
                return 0;
            }
        }
        link.dfs()
    }

    fn dfs(&self) -> i32 {
        let mut res = self.count;
        for next in self.children.values() {
            res += next.dfs();
        }
        res
    }

    fn erase(&mut self, word: String) {
        let mut link = self;
        for c in word.chars() {
            link = link.children.entry(c).or_default();
        }
        link.count -= 1;
    }
}

#[test]
fn test() {
    let mut obj = Trie::new();
    obj.insert("apple".to_string());
    obj.insert("apple".to_string());
    assert_eq!(obj.count_words_equal_to("apple".to_string()), 2);
    assert_eq!(obj.count_words_starting_with("app".to_string()), 2);
    obj.erase("apple".to_string());
    assert_eq!(obj.count_words_equal_to("apple".to_string()), 1);
    assert_eq!(obj.count_words_starting_with("app".to_string()), 1);
    obj.erase("apple".to_string());
    assert_eq!(obj.count_words_starting_with("app".to_string()), 0);
}
