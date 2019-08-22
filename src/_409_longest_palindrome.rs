struct Solution;

use std::collections::HashSet;

impl Solution {
    fn longest_palindrome(s: String) -> i32 {
        let mut hs: HashSet<char> = HashSet::new();
        let mut half = 0;
        for c in s.chars() {
            if hs.contains(&c) {
                hs.remove(&c);
                half += 1;
            } else {
                hs.insert(c);
            }
        }
        if hs.is_empty() {
            2 * half
        } else {
            2 * half + 1
        }
    }
}

#[test]
fn test() {
    let s = "abccccdd".to_string();
    assert_eq!(Solution::longest_palindrome(s), 7);
}
