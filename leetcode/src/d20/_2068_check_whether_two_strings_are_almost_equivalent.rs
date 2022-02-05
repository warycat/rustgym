struct Solution;

use std::collections::HashMap;

impl Solution {
    fn check_almost_equivalent(word1: String, word2: String) -> bool {
        let mut hm1: HashMap<char, i32> = HashMap::new();
        let mut hm2: HashMap<char, i32> = HashMap::new();
        for c in word1.chars() {
            *hm1.entry(c).or_default() += 1;
        }
        for c in word2.chars() {
            *hm2.entry(c).or_default() += 1;
        }
        for c in 'a'..='z' {
            let x1 = *hm1.entry(c).or_default();
            let x2 = *hm2.entry(c).or_default();
            if (x1 - x2).abs() > 3 {
                return false;
            }
        }
        true
    }
}

#[test]
fn test() {
    let word1 = "aaaa".to_string();
    let word2 = "bccb".to_string();
    let res = false;
    assert_eq!(Solution::check_almost_equivalent(word1, word2), res);
    let word1 = "abcdeef".to_string();
    let word2 = "abaaacc".to_string();
    let res = true;
    assert_eq!(Solution::check_almost_equivalent(word1, word2), res);
    let word1 = "cccddabba".to_string();
    let word2 = "babababab".to_string();
    let res = true;
    assert_eq!(Solution::check_almost_equivalent(word1, word2), res);
}
