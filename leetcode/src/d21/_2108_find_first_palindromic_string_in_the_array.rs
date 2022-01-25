struct Solution;

impl Solution {
    fn first_palindrome(words: Vec<String>) -> String {
        let n = words.len();
        for i in 0..n {
            if Solution::is_palindrom(&words[i]) {
                return words[i].to_string();
            }
        }
        "".to_string()
    }

    fn is_palindrom(s: &str) -> bool {
        let rev: String = s.chars().rev().collect();
        s == rev
    }
}

#[test]
fn test() {
    let words = vec_string!["abc", "car", "ada", "racecar", "cool"];
    let res = "ada".to_string();
    assert_eq!(Solution::first_palindrome(words), res);
    let words = vec_string!["notapalindrome", "racecar"];
    let res = "racecar".to_string();
    assert_eq!(Solution::first_palindrome(words), res);
    let words = vec_string!["def", "ghi"];
    let res = "".to_string();
    assert_eq!(Solution::first_palindrome(words), res);
}
