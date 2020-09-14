struct Solution;

use std::collections::HashMap;

const MOD: i64 = 1_000_000_007;

impl Solution {
    fn unique_letter_string(s: String) -> i32 {
        let n = s.len();
        let s: Vec<char> = s.chars().collect();
        let mut idx: HashMap<char, Vec<usize>> = HashMap::new();
        for i in 0..n {
            idx.entry(s[i]).or_default().push(i + 1);
        }
        let mut res = 0;
        for v in idx.values() {
            let m = v.len();
            for i in 0..m {
                let prev = if i > 0 { v[i - 1] } else { 0 };
                let next = if i + 1 < m { v[i + 1] } else { n + 1 };
                res += ((v[i] - prev) * (next - v[i])) as i64;
                res %= MOD;
            }
        }
        res as i32
    }
}

#[test]
fn test() {
    let s = "ABC".to_string();
    let res = 10;
    assert_eq!(Solution::unique_letter_string(s), res);
    let s = "ABA".to_string();
    let res = 8;
    assert_eq!(Solution::unique_letter_string(s), res);
    let s = "LEETCODE".to_string();
    let res = 92;
    assert_eq!(Solution::unique_letter_string(s), res);
}
