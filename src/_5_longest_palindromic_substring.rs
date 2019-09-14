struct Solution;

use std::iter::FromIterator;

impl Solution {
    fn longest_palindrome(s: String) -> String {
        let n = s.len();
        if n == 0 {
            return "".to_string();
        }
        let s: Vec<char> = s.chars().collect();
        let mut start = 0;
        let mut end = 0;
        for i in 0..n {
            let mut left = i;
            let mut right = i;
            while right + 1 < n && s[right + 1] == s[left] {
                right += 1;
            }
            while left > 0 && right < n - 1 && s[left - 1] == s[right + 1] {
                left -= 1;
                right += 1;
            }
            if right - left > end - start {
                start = left;
                end = right;
            }
        }
        String::from_iter(&s[start..=end])
    }
}

#[test]
fn test() {
    let s = "babad".to_string();
    let res = "bab".to_string();
    assert_eq!(Solution::longest_palindrome(s), res);
}
