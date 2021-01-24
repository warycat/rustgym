struct Solution;
use std::collections::HashSet;

impl Solution {
    fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
        let hs: HashSet<char> = allowed.chars().collect();
        words
            .into_iter()
            .filter(|w| w.chars().all(|c| hs.contains(&c)))
            .count() as i32
    }
}

#[test]
fn test() {
    let allowed = "ab".to_string();
    let words = vec_string!["ad", "bd", "aaab", "baa", "badab"];
    let res = 2;
    assert_eq!(Solution::count_consistent_strings(allowed, words), res);
    let allowed = "abc".to_string();
    let words = vec_string!["a", "b", "c", "ab", "ac", "bc", "abc"];
    let res = 7;
    assert_eq!(Solution::count_consistent_strings(allowed, words), res);
    let allowed = "cad".to_string();
    let words = vec_string!["cc", "acd", "b", "ba", "bac", "bad", "ac", "d"];
    let res = 4;
    assert_eq!(Solution::count_consistent_strings(allowed, words), res);
}
