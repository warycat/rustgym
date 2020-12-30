struct Solution;

use std::collections::HashMap;

impl Solution {
    fn min_insertions(s: String) -> i32 {
        let n = s.len();
        let s: Vec<char> = s.chars().collect();
        let mut memo: HashMap<(usize, usize), i32> = HashMap::new();
        Self::dp(0, n, &mut memo, &s)
    }
    fn dp(start: usize, end: usize, memo: &mut HashMap<(usize, usize), i32>, s: &[char]) -> i32 {
        let n = end - start;
        if n < 2 {
            return 0;
        }
        if let Some(&res) = memo.get(&(start, end)) {
            return res;
        }
        let res = if s[start] == s[end - 1] {
            Self::dp(start + 1, end - 1, memo, s)
        } else {
            1 + Self::dp(start, end - 1, memo, s).min(Self::dp(start + 1, end, memo, s))
        };
        memo.insert((start, end), res);
        res
    }
}

#[test]
fn test() {
    let s = "zzazz".to_string();
    let res = 0;
    assert_eq!(Solution::min_insertions(s), res);
    let s = "mbadm".to_string();
    let res = 2;
    assert_eq!(Solution::min_insertions(s), res);
    let s = "leetcode".to_string();
    let res = 5;
    assert_eq!(Solution::min_insertions(s), res);
    let s = "g".to_string();
    let res = 0;
    assert_eq!(Solution::min_insertions(s), res);
    let s = "no".to_string();
    let res = 1;
    assert_eq!(Solution::min_insertions(s), res);
}
