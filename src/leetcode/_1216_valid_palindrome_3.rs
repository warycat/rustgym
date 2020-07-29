struct Solution;

use std::collections::HashMap;

impl Solution {
    fn is_valid_palindrome(s: String, k: i32) -> bool {
        let mut memo: HashMap<&str, usize> = HashMap::new();
        let k = k as usize;
        Self::dp(&s, &mut memo) <= k
    }

    fn dp<'a>(s: &'a str, memo: &mut HashMap<&'a str, usize>) -> usize {
        if s.len() < 2 {
            return 0;
        }
        if let Some(&res) = memo.get(&s) {
            return res;
        }
        let n = s.len();
        let res = if s.chars().next().unwrap() == s.chars().next_back().unwrap() {
            Self::dp(&s[1..n - 1], memo)
        } else {
            Self::dp(&s[0..n - 1], memo).min(Self::dp(&s[1..n], memo)) + 1
        };
        memo.insert(s, res);
        res
    }
}

#[test]
fn test() {
    let s = "abcdeca".to_string();
    let k = 2;
    let res = true;
    assert_eq!(Solution::is_valid_palindrome(s, k), res);
}
