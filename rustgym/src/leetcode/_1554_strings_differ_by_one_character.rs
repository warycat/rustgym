struct Solution;
use std::collections::HashSet;

impl Solution {
    fn differ_by_one(dict: Vec<String>) -> bool {
        let n = dict.len();
        let m = dict[0].len();
        for j in 0..m {
            let mut hs: HashSet<String> = HashSet::new();
            for i in 0..n {
                let left = &dict[i][..j];
                let right = &dict[i][j + 1..];
                if !hs.insert(format!("{}{}", left, right)) {
                    return true;
                }
            }
        }
        false
    }
}
#[test]
fn test() {
    let dict = vec_string!["abcd", "acbd", "aacd"];
    let res = true;
    assert_eq!(Solution::differ_by_one(dict), res);
    let dict = vec_string!["ab", "cd", "yz"];
    let res = false;
    assert_eq!(Solution::differ_by_one(dict), res);
    let dict = vec_string!["abcd", "cccc", "abyd", "abab"];
    let res = true;
    assert_eq!(Solution::differ_by_one(dict), res);
}
