struct Solution;
use std::collections::HashSet;
use std::iter::FromIterator;

impl Solution {
    fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        let n = begin_word.len();
        let mut unused_set: HashSet<Vec<u8>> = word_list
            .into_iter()
            .map(|s| s.as_bytes().to_vec())
            .collect();
        let begin_word = begin_word.as_bytes().to_vec();
        let end_word = end_word.as_bytes().to_vec();
        if !unused_set.contains(&end_word) {
            return 0;
        }
        let begin_list = vec![begin_word];
        let end_list = vec![end_word];
        let mut begin_set: HashSet<Vec<u8>> = HashSet::from_iter(begin_list);
        let mut end_set: HashSet<Vec<u8>> = HashSet::from_iter(end_list);
        let mut left_set: &mut HashSet<Vec<u8>> = &mut begin_set;
        let mut right_set: &mut HashSet<Vec<u8>> = &mut end_set;
        let mut ladder = 1;
        while !left_set.is_empty() {
            ladder += 1;
            let mut next_set: HashSet<Vec<u8>> = HashSet::new();
            for s in left_set.iter() {
                let mut v: Vec<u8> = s.to_vec();
                for i in 0..n {
                    let c = v[i];
                    for j in 0..26 {
                        v[i] = b'a' + j;
                        if right_set.contains(&v) {
                            return ladder;
                        }
                        if unused_set.contains(&v) {
                            unused_set.remove(&v);
                            next_set.insert(v.to_vec());
                        }
                    }
                    v[i] = c;
                }
            }
            *left_set = next_set;
            if left_set.len() > right_set.len() {
                std::mem::swap(&mut left_set, &mut right_set)
            }
        }
        0
    }
}

#[test]
fn test() {
    let begin_word = "hit".to_string();
    let end_word = "cog".to_string();
    let word_list = vec_string!["hot", "dot", "dog", "lot", "log", "cog"];
    assert_eq!(Solution::ladder_length(begin_word, end_word, word_list), 5);
    let begin_word = "hit".to_string();
    let end_word = "cog".to_string();
    let word_list = vec_string!["hot", "dot", "dog", "lot", "log"];
    assert_eq!(Solution::ladder_length(begin_word, end_word, word_list), 0);
}
