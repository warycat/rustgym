use std::collections::HashMap;

#[derive(PartialEq, Eq, Clone, Debug, Default)]
struct Trie {
    children: HashMap<char, Trie>,
    end: bool,
}

impl Trie {
    fn new() -> Self {
        Self::default()
    }

    fn insert(&mut self, word: String) {
        let mut link = self;
        for c in word.chars().rev() {
            link = link.children.entry(c).or_default();
        }
        link.end = true;
    }

    fn search(&self, stream: &[char]) -> bool {
        let mut link = self;
        for c in stream.iter().rev() {
            if let Some(next) = link.children.get(c) {
                link = next;
                if next.end {
                    return true;
                }
            } else {
                return false;
            }
        }
        false
    }
}

struct StreamChecker {
    trie: Trie,
    stream: Vec<char>,
}

impl StreamChecker {
    fn new(words: Vec<String>) -> Self {
        let mut trie = Trie::new();
        for s in words {
            trie.insert(s);
        }
        let stream = vec![];
        StreamChecker { trie, stream }
    }

    fn query(&mut self, letter: char) -> bool {
        self.stream.push(letter);
        self.trie.search(&self.stream)
    }
}

#[test]
fn test() {
    let words = vec_string!["cd", "f", "kl"];
    let mut obj = StreamChecker::new(words);
    assert_eq!(obj.query('a'), false);
    assert_eq!(obj.query('b'), false);
    assert_eq!(obj.query('c'), false);
    assert_eq!(obj.query('d'), true);
    assert_eq!(obj.query('e'), false);
    assert_eq!(obj.query('f'), true);
    assert_eq!(obj.query('g'), false);
    assert_eq!(obj.query('h'), false);
    assert_eq!(obj.query('i'), false);
    assert_eq!(obj.query('j'), false);
    assert_eq!(obj.query('k'), false);
    assert_eq!(obj.query('l'), true);
}
