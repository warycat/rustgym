struct Solution;

impl Solution {
    fn valid_palindrome(s: String) -> bool {
        let v: &str = s.as_str();
        if let Some(s) = Solution::is_palidrome(v) {
            let sl: &str = &s[1..];
            let sr: &str = &s[..s.len() - 1];
            Self::is_palidrome(sl).is_none() || Self::is_palidrome(sr).is_none()
        } else {
            true
        }
    }

    fn is_palidrome(v: &str) -> Option<&str> {
        let n = v.len();
        if n <= 1 {
            return None;
        }
        if v.chars().next() != v.chars().last() {
            Some(v)
        } else {
            Self::is_palidrome(&v[1..n - 1])
        }
    }
}

#[test]
fn test() {
    let s = "aba".to_string();
    assert_eq!(Solution::valid_palindrome(s), true);
    let s = "abca".to_string();
    assert_eq!(Solution::valid_palindrome(s), true);
    let s = "accca".to_string();
    assert_eq!(Solution::valid_palindrome(s), true);
}
