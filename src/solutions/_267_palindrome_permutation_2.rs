struct Solution;
use std::collections::HashMap;

impl Solution {
    fn generate_palindromes(s: String) -> Vec<String> {
        let mut hm: HashMap<char, usize> = HashMap::new();
        for c in s.chars() {
            *hm.entry(c).or_default() += 1;
        }
        let mut odd = 0;
        for &count in hm.values() {
            if count % 2 == 1 {
                odd += 1;
            }
        }
        if odd > 1 {
            return vec![];
        }
        let mut left: Vec<char> = vec![];
        let mut mid: Vec<char> = vec![];
        for (&c, &count) in &hm {
            if count % 2 == 1 {
                mid.push(c);
            }
            for _ in 0..count / 2 {
                left.push(c);
            }
        }
        let n = left.len();
        left.sort_unstable();
        let mut res: Vec<String> = vec![];
        let mut used: Vec<bool> = vec![false; n];
        let mut cur: Vec<char> = vec![];
        Self::dfs(&mut cur, &mut used, &mut res, &mid, &left, n);
        res.into_iter().collect()
    }

    fn dfs(
        cur: &mut Vec<char>,
        used: &mut Vec<bool>,
        all: &mut Vec<String>,
        mid: &[char],
        left: &[char],
        n: usize,
    ) {
        if cur.len() == n {
            let mut v = vec![];
            let mut l = cur.to_vec();
            let mut m = mid.to_vec();
            let mut r = cur.to_vec();
            r.reverse();
            v.append(&mut l);
            v.append(&mut m);
            v.append(&mut r);
            all.push(v.into_iter().collect());
        } else {
            for i in 0..n {
                if used[i] {
                    continue;
                }
                if i > 0 && left[i] == left[i - 1] && !used[i - 1] {
                    continue;
                }
                used[i] = true;
                cur.push(left[i]);
                Self::dfs(cur, used, all, mid, left, n);
                used[i] = false;
                cur.pop();
            }
        }
    }
}

#[test]
fn test() {
    let s = "aabb".to_string();
    let res = vec_string!["abba", "baab"];
    assert_eq!(Solution::generate_palindromes(s), res);
    let s = "abc".to_string();
    let res: Vec<String> = vec![];
    assert_eq!(Solution::generate_palindromes(s), res);
}
