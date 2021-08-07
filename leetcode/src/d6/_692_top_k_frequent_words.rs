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
            *hm.entry(w).or_default() += 1;
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
    let words: Vec<String> = vec_string!["i", "love", "leetcode", "i", "love", "coding"];
    let res: Vec<String> = vec_string!["i", "love"];
    let k = 2;
    assert_eq!(Solution::top_k_frequent(words, k), res);
    let words: Vec<String> =
        vec_string!["the", "day", "is", "sunny", "the", "the", "the", "sunny", "is", "is"];
    let res: Vec<String> = vec_string!["the", "is", "sunny", "day"];
    let k = 4;
    assert_eq!(Solution::top_k_frequent(words, k), res);
}
