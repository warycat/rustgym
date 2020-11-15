struct Solution;

use std::collections::HashMap;

impl Solution {
    fn close_strings(word1: String, word2: String) -> bool {
        let mut hm1: HashMap<char, usize> = HashMap::new();
        let mut hm2: HashMap<char, usize> = HashMap::new();

        for c in word1.chars() {
            *hm1.entry(c).or_default() += 1;
        }
        for c in word2.chars() {
            *hm2.entry(c).or_default() += 1;
        }
        let mut keys1: Vec<char> = hm1.keys().copied().collect();
        let mut keys2: Vec<char> = hm2.keys().copied().collect();
        keys1.sort_unstable();
        keys2.sort_unstable();
        let mut values1: Vec<usize> = hm1.values().copied().collect();
        let mut values2: Vec<usize> = hm2.values().copied().collect();
        values1.sort_unstable();
        values2.sort_unstable();
        keys1 == keys2 && values1 == values2
    }
}

#[test]
fn test() {
    let word1 = "abc".to_string();
    let word2 = "bca".to_string();
    let res = true;
    assert_eq!(Solution::close_strings(word1, word2), res);
    let word1 = "a".to_string();
    let word2 = "aa".to_string();
    let res = false;
    assert_eq!(Solution::close_strings(word1, word2), res);
    let word1 = "cabbba".to_string();
    let word2 = "abbccc".to_string();
    let res = true;
    assert_eq!(Solution::close_strings(word1, word2), res);
    let word1 = "cabbba".to_string();
    let word2 = "aabbss".to_string();
    let res = false;
    assert_eq!(Solution::close_strings(word1, word2), res);
}
