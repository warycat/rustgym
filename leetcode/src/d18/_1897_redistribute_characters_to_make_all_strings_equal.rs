struct Solution;

use std::collections::HashMap;

impl Solution {
    fn make_equal(words: Vec<String>) -> bool {
        let n = words.len();
        let mut hm: HashMap<char, usize> = HashMap::new();
        for word in words {
            for c in word.chars() {
                *hm.entry(c).or_default() += 1;
            }
        }
        for v in hm.values() {
            if v % n != 0 {
                return false;
            }
        }
        true
    }
}

#[test]
fn test() {
    let words = vec_string!["abc", "aabc", "bc"];
    let res = true;
    assert_eq!(Solution::make_equal(words), res);
    let words = vec_string!["ab", "a"];
    let res = false;
    assert_eq!(Solution::make_equal(words), res);
}
