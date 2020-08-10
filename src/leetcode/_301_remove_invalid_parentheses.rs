struct Solution;

use std::collections::HashSet;

impl Solution {
    fn remove_invalid_parentheses(s: String) -> Vec<String> {
        let mut cur = vec![];
        let s: Vec<char> = s.chars().collect();
        let n = s.len();
        let mut min = std::usize::MAX;
        let mut res: HashSet<String> = HashSet::new();
        Self::dfs(0, 0, 0, &mut cur, &mut res, &mut min, &s, n);
        res.into_iter().collect()
    }

    fn dfs(
        start: usize,
        left: usize,
        remove: usize,
        cur: &mut Vec<char>,
        all: &mut HashSet<String>,
        min: &mut usize,
        s: &[char],
        n: usize,
    ) {
        if start == n {
            if left != 0 {
                return;
            }
            if remove > *min {
                return;
            }
            if remove < *min {
                *min = remove;
                all.clear();
            }
            let s = cur.iter().copied().collect();
            all.insert(s);
        } else {
            match s[start] {
                '(' => {
                    cur.push('(');
                    Self::dfs(start + 1, left + 1, remove, cur, all, min, s, n);
                    cur.pop();
                    Self::dfs(start + 1, left, remove + 1, cur, all, min, s, n);
                }
                ')' => {
                    if left > 0 {
                        cur.push(')');
                        Self::dfs(start + 1, left - 1, remove, cur, all, min, s, n);
                        cur.pop();
                    }
                    Self::dfs(start + 1, left, remove + 1, cur, all, min, s, n);
                }
                _ => {
                    cur.push(s[start]);
                    Self::dfs(start + 1, left, remove, cur, all, min, s, n);
                    cur.pop();
                }
            }
        }
    }
}

#[test]
fn test() {
    let s = "()())()".to_string();
    let mut res = vec_string!["()()()", "(())()"];
    let mut ans = Solution::remove_invalid_parentheses(s);
    ans.sort();
    res.sort();
    assert_eq!(ans, res);
    let s = "(a)())()".to_string();
    let mut res = vec_string!["(a)()()", "(a())()"];
    let mut ans = Solution::remove_invalid_parentheses(s);
    ans.sort();
    res.sort();
    assert_eq!(ans, res);
    let s = ")(".to_string();
    let mut res = vec_string![""];
    let mut ans = Solution::remove_invalid_parentheses(s);
    ans.sort();
    res.sort();
    assert_eq!(ans, res);
    let s = ")(f".to_string();
    let mut res = vec_string!["f"];
    let mut ans = Solution::remove_invalid_parentheses(s);
    ans.sort();
    res.sort();
    assert_eq!(ans, res);
}
