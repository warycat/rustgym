struct Solution;

use std::collections::HashMap;

impl Solution {
    fn min_cut(s: String) -> i32 {
        let n = s.len();
        let s: Vec<char> = s.chars().collect();
        let mut memo: HashMap<(usize, usize), i32> = HashMap::new();
        Self::dp(0, n, &mut memo, &s)
    }

    fn dp(start: usize, end: usize, memo: &mut HashMap<(usize, usize), i32>, s: &[char]) -> i32 {
        if let Some(&res) = memo.get(&(start, end)) {
            return res;
        }
        let res = if Self::is_palindrome(start, end, s) {
            0
        } else {
            let mut res = std::i32::MAX;
            for i in start + 1..end {
                if Self::is_palindrome(start, i, s) {
                    res = res.min(1 + Self::dp(i, end, memo, s));
                }
            }
            res
        };
        memo.insert((start, end), res);
        res
    }

    fn is_palindrome(start: usize, end: usize, s: &[char]) -> bool {
        !s[start..end]
            .iter()
            .zip(s[start..end].iter().rev())
            .any(|(a, b)| a != b)
    }
}

#[test]
fn test() {
    let s = "aab".to_string();
    let res = 1;
    assert_eq!(Solution::min_cut(s), res);
    let s = "coder".to_string();
    let res = 4;
    assert_eq!(Solution::min_cut(s), res);
}
