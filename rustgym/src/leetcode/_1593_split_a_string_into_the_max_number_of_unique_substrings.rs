struct Solution;

use std::collections::HashSet;

impl Solution {
    fn max_unique_split(s: String) -> i32 {
        let n = s.len();
        let mut visited: HashSet<String> = HashSet::new();
        let mut res = 0;
        Self::dfs(0, 0, &mut visited, &mut res, &s, n);
        res as i32
    }

    fn dfs(
        start: usize,
        cur: usize,
        visited: &mut HashSet<String>,
        max: &mut usize,
        s: &str,
        n: usize,
    ) {
        if start == n {
            *max = (*max).max(cur);
        } else {
            for i in start..n {
                if !visited.contains(&s[start..=i]) {
                    visited.insert(s[start..=i].to_string());
                    Self::dfs(i + 1, cur + 1, visited, max, s, n);
                    visited.remove(&s[start..=i]);
                }
            }
        }
    }
}

#[test]
fn test() {
    let s = "ababccc".to_string();
    let res = 5;
    assert_eq!(Solution::max_unique_split(s), res);
    let s = "aba".to_string();
    let res = 2;
    assert_eq!(Solution::max_unique_split(s), res);
    let s = "aa".to_string();
    let res = 1;
    assert_eq!(Solution::max_unique_split(s), res);
}
