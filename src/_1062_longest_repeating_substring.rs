struct Solution;
use std::collections::HashSet;

impl Solution {
    fn longest_repeating_substring(s: String) -> i32 {
        let n = s.len();
        let mut res = 0;
        let mut hs: HashSet<&str> = HashSet::new();
        for k in 1..n {
            for i in 0..=n - k {
                if !hs.insert(&s[i..i + k]) {
                    res = res.max(k);
                    break;
                }
            }
            if res < k {
                break;
            }
        }
        res as i32
    }
}

#[test]
fn test() {
    let s = "abcd".to_string();
    let res = 0;
    assert_eq!(Solution::longest_repeating_substring(s), res);
    let s = "abbaba".to_string();
    let res = 2;
    assert_eq!(Solution::longest_repeating_substring(s), res);
    let s = "aabcaabdaab".to_string();
    let res = 3;
    assert_eq!(Solution::longest_repeating_substring(s), res);
    let s = "aaaaa".to_string();
    let res = 4;
    assert_eq!(Solution::longest_repeating_substring(s), res);
}
