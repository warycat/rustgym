struct Solution;

use std::collections::HashMap;

impl Solution {
    fn length_of_longest_substring(s: String) -> i32 {
        let mut hm: HashMap<char, usize> = HashMap::new();
        let mut max: usize = 0;
        let mut l: usize = 0;
        for (r, c) in s.char_indices() {
            if let Some(end) = hm.insert(c, r) {
                l = usize::max(l, end + 1);
            }
            max = usize::max(r - l + 1, max);
        }
        max as i32
    }
}

#[test]
fn test() {
    let s = " ".to_string();
    assert_eq!(Solution::length_of_longest_substring(s), 1);
    let s = "abba".to_string();
    assert_eq!(Solution::length_of_longest_substring(s), 2);
    let s = "abcabcbb".to_string();
    assert_eq!(Solution::length_of_longest_substring(s), 3);
    let s = "bbbb".to_string();
    assert_eq!(Solution::length_of_longest_substring(s), 1);
    let s = "pwwkew".to_string();
    assert_eq!(Solution::length_of_longest_substring(s), 3);
}
