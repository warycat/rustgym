struct Solution;

use std::collections::HashMap;

impl Solution {
    fn count_words(words1: Vec<String>, words2: Vec<String>) -> i32 {
        let mut hm1: HashMap<String, usize> = HashMap::new();
        let mut hm2: HashMap<String, usize> = HashMap::new();
        for w in words1 {
            *hm1.entry(w).or_default() += 1;
        }
        for w in words2 {
            *hm2.entry(w).or_default() += 1;
        }
        let mut res = 0;
        for (k, v) in hm1 {
            if v == 1 && *hm2.get(&k).unwrap_or(&0) == 1 {
                res += 1;
            }
        }
        res
    }
}

#[test]
fn test() {
    let words1 = vec_string!["leetcode", "is", "amazing", "as", "is"];
    let words2 = vec_string!["amazing", "leetcode", "is"];
    let res = 2;
    assert_eq!(Solution::count_words(words1, words2), res);
    let words1 = vec_string!["b", "bb", "bbb"];
    let words2 = vec_string!["a", "aa", "aaa"];
    let res = 0;
    assert_eq!(Solution::count_words(words1, words2), res);
    let words1 = vec_string!["a", "ab"];
    let words2 = vec_string!["a", "a", "a", "ab"];
    let res = 1;
    assert_eq!(Solution::count_words(words1, words2), res);
}
