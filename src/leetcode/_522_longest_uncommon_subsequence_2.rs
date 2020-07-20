struct Solution;
use std::cmp::Reverse;

trait IsSubSequenceOf {
    fn is_subsequence_of(&self, s: &str) -> bool;
}

impl IsSubSequenceOf for String {
    fn is_subsequence_of(&self, s: &str) -> bool {
        let mut it = self.chars().peekable();
        for c in s.chars() {
            if let Some(&first) = it.peek() {
                if first == c {
                    it.next();
                }
            }
        }
        it.next().is_none()
    }
}

impl Solution {
    fn find_lu_slength(mut strs: Vec<String>) -> i32 {
        let n = strs.len();
        strs.sort_by_key(|s| Reverse(s.len()));
        for i in 0..n {
            let mut count = 0;
            for j in 0..n {
                if i != j {
                    if !strs[i].is_subsequence_of(&strs[j]) {
                        count += 1;
                    }
                }
            }
            if count == n - 1 {
                return strs[i].len() as i32;
            }
        }
        -1
    }
}

#[test]
fn test() {
    let strs = vec_string!["aba", "cdc", "eae"];
    let res = 3;
    assert_eq!(Solution::find_lu_slength(strs), res);
}
