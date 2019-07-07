struct Solution {}

impl Solution {
    fn is_palindrome(s: String) -> bool {
        let s: Vec<char> = s
            .chars()
            .filter(|c| c.is_ascii_alphanumeric())
            .map(|c| c.to_ascii_lowercase())
            .collect();
        let a: String = s.clone().iter().collect();
        let b: String = s.clone().iter().rev().collect();
        a == b
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::is_palindrome("A man, a plan, a canal: Panama".to_string()),
        true
    );
    assert_eq!(Solution::is_palindrome("race a car".to_string()), false);
}
