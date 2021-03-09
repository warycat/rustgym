struct Solution;

use std::cmp::Ordering::Less;

impl Solution {
    fn split_looped_string(mut strs: Vec<String>) -> String {
        let n = strs.len();
        strs = strs
            .into_iter()
            .map(|s| {
                let a = s.to_string();
                let b = s.chars().rev().collect();
                if let Less = a.cmp(&b) {
                    b
                } else {
                    a
                }
            })
            .collect();
        let mut max = strs.join("");
        for i in 0..n {
            let left = strs[..i].join("");
            let right = strs[i + 1..].join("");
            let m = strs[i].len();
            let a = strs[i].to_string();
            let b = a.chars().rev().collect();
            for s in &[a, b] {
                for j in 0..m {
                    let mut t = "".to_string();
                    t.push_str(&s[j..]);
                    t.push_str(&right);
                    t.push_str(&left);
                    t.push_str(&s[..j]);
                    max = if let Less = max.cmp(&t) { t } else { max };
                }
            }
        }
        max
    }
}

#[test]
fn test() {
    let strs = vec_string!["abc", "xyz"];
    let res = "zyxcba".to_string();
    assert_eq!(Solution::split_looped_string(strs), res);
    let strs = vec_string!["a", "b", "c"];
    let res = "cab".to_string();
    assert_eq!(Solution::split_looped_string(strs), res);
}
