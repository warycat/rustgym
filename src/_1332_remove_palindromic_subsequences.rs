struct Solution;

trait Palindrome {
    fn is_palindrome(&self) -> bool;
}

impl Palindrome for String {
    fn is_palindrome(&self) -> bool {
        let mut i = 0;
        let n = self.len();
        let mut j = n - 1;
        let v: Vec<char> = self.chars().collect();
        while i <= j {
            if v[i] != v[j] {
                return false;
            }
            i += 1;
            j -= 1;
        }
        true
    }
}

impl Solution {
    fn remove_palindrome_sub(s: String) -> i32 {
        if s.is_empty() {
            0
        } else {
            if s.is_palindrome() {
                1
            } else {
                2
            }
        }
    }
}

#[test]
fn test() {
    let s = "ababa".to_string();
    let res = 1;
    assert_eq!(Solution::remove_palindrome_sub(s), res);
    let s = "abb".to_string();
    let res = 2;
    assert_eq!(Solution::remove_palindrome_sub(s), res);
    let s = "baabb".to_string();
    let res = 2;
    assert_eq!(Solution::remove_palindrome_sub(s), res);
    let s = "".to_string();
    let res = 0;
    assert_eq!(Solution::remove_palindrome_sub(s), res);
}
