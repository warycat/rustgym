struct Solution;

use std::collections::HashSet;

impl Solution {
    fn num_jewels_in_stones(j: String, s: String) -> i32 {
        let mut hs: HashSet<char> = HashSet::new();
        for c in j.chars() {
            hs.insert(c);
        }
        let mut res = 0;
        for c in s.chars() {
            if hs.contains(&c) {
                res += 1;
            }
        }
        res
    }
}

#[test]
fn test() {
    let j = "aA".to_string();
    let s = "aAAbbbb".to_string();
    assert_eq!(Solution::num_jewels_in_stones(j, s), 3);
    let j = "z".to_string();
    let s = "ZZ".to_string();
    assert_eq!(Solution::num_jewels_in_stones(j, s), 0);
}
