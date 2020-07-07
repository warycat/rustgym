struct Solution;
use std::iter::once;

impl Solution {
    fn shortest_palindrome(s: String) -> String {
        let n = s.len();
        let s_new: Vec<char> = s.chars().chain(once('#')).chain(s.chars().rev()).collect();
        let n_new = s_new.len();
        let mut f = vec![0; n_new];
        for i in 1..n_new {
            let mut t = f[i - 1];
            while t > 0 && s_new[i] != s_new[t] {
                t = f[t - 1];
            }
            if s_new[i] == s_new[t] {
                t += 1;
            }
            f[i] = t;
        }
        s.chars()
            .rev()
            .take(n - f.last().unwrap())
            .chain(s.chars())
            .collect()
    }
}

#[test]
fn test() {
    let s = "aacecaaa".to_string();
    let res = "aaacecaaa".to_string();
    assert_eq!(Solution::shortest_palindrome(s), res);
    let s = "abcd".to_string();
    let res = "dcbabcd".to_string();
    assert_eq!(Solution::shortest_palindrome(s), res);
}
