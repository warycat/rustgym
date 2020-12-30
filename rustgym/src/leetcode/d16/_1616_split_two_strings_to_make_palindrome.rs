struct Solution;

impl Solution {
    fn check_palindrome_formation(a: String, b: String) -> bool {
        let a: Vec<char> = a.chars().collect();
        let b: Vec<char> = b.chars().collect();
        Self::check(&a, &b) || Self::check(&b, &a)
    }
    fn is_palindrome(s: &[char]) -> bool {
        s.iter().zip(s.iter().rev()).all(|(a, b)| a == b)
    }
    fn check(a: &[char], b: &[char]) -> bool {
        let mut i = 0;
        let n = b.len();
        while a[i] == b[n - 1 - i] {
            i += 1;
        }
        Self::is_palindrome(&a[i..n - i]) || Self::is_palindrome(&b[i..n - i])
    }
}

#[test]
fn test() {
    let a = "x".to_string();
    let b = "y".to_string();
    let res = true;
    assert_eq!(Solution::check_palindrome_formation(a, b), res);
    let a = "abdef".to_string();
    let b = "fecab".to_string();
    let res = true;
    assert_eq!(Solution::check_palindrome_formation(a, b), res);
    let a = "ulacfd".to_string();
    let b = "jizalu".to_string();
    let res = true;
    assert_eq!(Solution::check_palindrome_formation(a, b), res);
    let a = "xbdef".to_string();
    let b = "xecab".to_string();
    let res = false;
    assert_eq!(Solution::check_palindrome_formation(a, b), res);
}
