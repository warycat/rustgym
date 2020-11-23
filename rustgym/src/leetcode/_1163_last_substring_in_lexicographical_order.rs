struct Solution;
use std::cmp::Ordering::*;

impl Solution {
    fn last_substring(s: String) -> String {
        let n = s.len();
        let mut i = 0;
        let mut j = 1;
        let mut k = 0;
        let v = s.as_bytes();
        while j + k < n {
            match v[i + k].cmp(&v[j + k]) {
                Equal => {
                    k += 1;
                }
                Greater => {
                    j = j + k + 1;
                    k = 0;
                }
                Less => {
                    i = j.max(i + k + 1);
                    j = i + 1;
                    k = 0;
                }
            }
        }
        s[i..].to_string()
    }
}

#[test]
fn test() {
    let s = "abab".to_string();
    let res = "bab".to_string();
    assert_eq!(Solution::last_substring(s), res);
    let s = "leetcode".to_string();
    let res = "tcode".to_string();
    assert_eq!(Solution::last_substring(s), res);
}
