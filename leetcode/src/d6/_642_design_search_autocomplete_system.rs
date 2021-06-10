use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::VecDeque;

type Pair = (Reverse<usize>, String);

#[derive(Default, Debug)]
struct Trie {
    children: HashMap<char, Trie>,
    count: usize,
}

impl Trie {
    fn new() -> Self {
        Trie::default()
    }

    fn insert(&mut self, s: String, time: usize) {
        let mut link = self;
        for c in s.chars() {
            link = link.children.entry(c).or_default();
        }
        link.count += time;
    }

    fn search(&self, s: &mut Vec<char>, top3: &mut BinaryHeap<Pair>) {
        if self.count > 0 {
            top3.push((Reverse(self.count), s.iter().copied().collect()));
        }
        if top3.len() > 3 {
            top3.pop();
        }
        for (&k, v) in &self.children {
            s.push(k);
            v.search(s, top3);
            s.pop();
        }
    }
}

struct AutocompleteSystem {
    buffer: Vec<char>,
    trie: Trie,
}

impl AutocompleteSystem {
    fn new(sentences: Vec<String>, times: Vec<i32>) -> Self {
        let mut trie = Trie::default();
        for (i, s) in sentences.into_iter().enumerate() {
            trie.insert(s, times[i] as usize);
        }
        let buffer: Vec<char> = vec![];
        AutocompleteSystem { buffer, trie }
    }

    fn input(&mut self, c: char) -> Vec<String> {
        if c == '#' {
            let s: String = self.buffer.drain(..).collect();
            self.trie.insert(s, 1);
            vec![]
        } else {
            let mut top3: BinaryHeap<Pair> = BinaryHeap::new();
            self.buffer.push(c);
            let mut link = &mut self.trie;
            for &c in self.buffer.iter() {
                link = link.children.entry(c).or_default();
            }
            link.search(&mut self.buffer, &mut top3);
            let mut res: VecDeque<String> = VecDeque::new();
            while let Some((_, s)) = top3.pop() {
                res.push_front(s);
            }
            res.into_iter().collect()
        }
    }
}

#[test]
fn test() {
    let sentences = vec_string!["i love you", "island", "ironman", "i love leetcode"];
    let times = vec![5, 3, 2, 2];
    let mut obj = AutocompleteSystem::new(sentences, times);
    assert_eq!(
        obj.input('i'),
        vec_string!["i love you", "island", "i love leetcode"]
    );
    assert_eq!(obj.input(' '), vec_string!["i love you", "i love leetcode"]);
    assert_eq!(obj.input('a'), vec_string![]);
    assert_eq!(obj.input('#'), vec_string![]);
    assert_eq!(
        obj.input('i'),
        vec_string!["i love you", "island", "i love leetcode"]
    );
    assert_eq!(
        obj.input(' '),
        vec_string!["i love you", "i love leetcode", "i a"]
    );
}
