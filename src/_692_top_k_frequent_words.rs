struct Solution;
use std::cmp::Ordering;
use std::collections::HashMap;
struct Pair<'a> {
    word: &'a str,
    freq: usize,
}

impl Solution {
    fn top_k_frequent(words: Vec<String>, k: i32) -> Vec<String> {
        let mut hm: HashMap<&str, usize> = HashMap::new();
        let mut v: Vec<Pair> = vec![];
        for w in words.iter() {
            *hm.entry(&w).or_default() += 1;
        }
        for (word, freq) in hm {
            v.push(Pair { word, freq });
        }
        v.sort_unstable_by(|a, b| match b.freq.cmp(&a.freq) {
            Ordering::Equal => a.word.cmp(b.word),
            e => e,
        });
        v.iter()
            .take(k as usize)
            .map(|x| x.word.to_string())
            .collect()
    }
}

#[test]
fn test() {
    let words: Vec<String> = ["i", "love", "leetcode", "i", "love", "coding"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    let res: Vec<String> = ["i", "love"].iter().map(|s| s.to_string()).collect();
    let k = 2;
    assert_eq!(Solution::top_k_frequent(words, k), res);
    let words: Vec<String> = [
        "the", "day", "is", "sunny", "the", "the", "the", "sunny", "is", "is",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();
    let res: Vec<String> = ["the", "is", "sunny", "day"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    let k = 4;
    assert_eq!(Solution::top_k_frequent(words, k), res);
}
