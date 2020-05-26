struct Solution;

impl Solution {
    fn break_palindrome(palindrome: String) -> String {
        let n = palindrome.len();
        let mut s: Vec<char> = palindrome.chars().collect();
        if n == 1 {
            return "".to_string();
        }
        for i in 0..n / 2 {
            if s[i] > 'a' {
                s[i] = 'a';
                return s.into_iter().collect();
            }
        }
        s[n - 1] = 'b';
        s.into_iter().collect()
    }
}

#[test]
fn test() {
    let palindrome = "abccba".to_string();
    let res = "aaccba".to_string();
    assert_eq!(Solution::break_palindrome(palindrome), res);
    let palindrome = "a".to_string();
    let res = "".to_string();
    assert_eq!(Solution::break_palindrome(palindrome), res);
}
