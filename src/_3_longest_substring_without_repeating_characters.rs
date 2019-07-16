struct Solution {}

use std::collections::HashSet;

impl Solution {
    fn length_of_longest_substring(s: String) -> i32 {
        let mut hs: HashSet<char> = HashSet::new();
        let n = s.len();
        let s: Vec<char> = s.chars().collect();
        let mut i: usize = 0;
        let mut j: usize = 0;
        let mut max: usize = 0;
        while i < n && j < n {
            if hs.contains(&s[j]) {
                hs.remove(&s[i]);
                i += 1;
            } else {
                hs.insert(s[j]);
                max = usize::max(max, j - i + 1);
                j += 1;
            }
        }
        max as i32
    }
}

#[test]
fn test() {
    let s = "abcabcbb".to_string();
    assert_eq!(Solution::length_of_longest_substring(s), 3);
    let s = "bbbb".to_string();
    assert_eq!(Solution::length_of_longest_substring(s), 1);
    let s = "pwwkew".to_string();
    assert_eq!(Solution::length_of_longest_substring(s), 3);
}
