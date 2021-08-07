struct Solution;

impl Solution {
    fn is_subsequence(s: String, t: String) -> bool {
        let mut i = 0;
        let mut j = 0;
        let n = s.len();
        let m = t.len();
        while i < n && j < m {
            if s[i..=i] == t[j..=j] {
                i += 1;
            }
            j += 1;
        }
        i == n
    }
}

#[test]
fn test() {
    let s = "abc".to_string();
    let t = "ahbgdc".to_string();
    assert_eq!(Solution::is_subsequence(s, t), true);
    let s = "axc".to_string();
    let t = "ahbgdc".to_string();
    assert_eq!(Solution::is_subsequence(s, t), false);
}
