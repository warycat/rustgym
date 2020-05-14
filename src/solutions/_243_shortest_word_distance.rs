struct Solution;

use std::collections::HashMap;
use std::i32;

impl Solution {
    fn shortest_distance(words: Vec<String>, word1: String, word2: String) -> i32 {
        let mut pos: HashMap<String, Vec<i32>> = HashMap::new();
        let mut min = i32::MAX;
        for i in 0..words.len() {
            let w = &words[i];
            if let Some(v) = pos.get_mut(w) {
                v.push(i as i32);
            } else {
                pos.insert(w.clone(), vec![i as i32]);
            }
        }
        let v1 = pos.get(&word1).unwrap();
        let v2 = pos.get(&word2).unwrap();
        for &i in v1 {
            for &j in v2 {
                min = i32::min(min, (i - j).abs());
            }
        }
        min
    }
}

#[test]
fn test() {
    let words: Vec<String> = vec_string!["practice", "makes", "perfect", "coding", "makes"];
    let word1 = "coding".to_string();
    let word2 = "practice".to_string();
    assert_eq!(Solution::shortest_distance(words, word1, word2), 3);
    let words: Vec<String> = vec_string!["practice", "makes", "perfect", "coding", "makes"];
    let word1 = "makes".to_string();
    let word2 = "coding".to_string();
    assert_eq!(Solution::shortest_distance(words, word1, word2), 1);
}
