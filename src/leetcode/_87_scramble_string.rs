struct Solution;

use std::collections::HashMap;

impl Solution {
    fn is_scramble(s1: String, s2: String) -> bool {
        let n = s1.len();
        let mut memo: HashMap<(usize, usize, usize, usize), bool> = HashMap::new();
        Self::dp(0, n, 0, n, &mut memo, &s1, &s2)
    }
    fn dp(
        l_start: usize,
        l_end: usize,
        r_start: usize,
        r_end: usize,
        memo: &mut HashMap<(usize, usize, usize, usize), bool>,
        s1: &str,
        s2: &str,
    ) -> bool {
        if let Some(&res) = memo.get(&(l_start, l_end, r_start, r_end)) {
            return res;
        }
        let res = if s1[l_start..l_end] == s2[r_start..r_end] {
            true
        } else {
            let mut res = false;
            let n = l_end - l_start;
            for i in 1..n {
                if Self::dp(l_start, l_start + i, r_start, r_start + i, memo, s1, s2)
                    && Self::dp(l_start + i, l_end, r_start + i, r_end, memo, s1, s2)
                    || Self::dp(l_start, l_start + i, r_end - i, r_end, memo, s1, s2)
                        && Self::dp(l_start + i, l_end, r_start, r_end - i, memo, s1, s2)
                {
                    res = true;
                    break;
                }
            }
            res
        };
        memo.insert((l_start, l_end, r_start, r_end), res);
        res
    }
}

#[test]
fn test() {
    let s1 = "great".to_string();
    let s2 = "rgeat".to_string();
    let res = true;
    assert_eq!(Solution::is_scramble(s1, s2), res);
    let s1 = "caebd".to_string();
    let s2 = "rgeat".to_string();
    let res = false;
    assert_eq!(Solution::is_scramble(s1, s2), res);
}
