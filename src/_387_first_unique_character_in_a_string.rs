struct Solution;

use std::collections::HashMap;

impl Solution {
    fn first_uniq_char(s: String) -> i32 {
        let mut hm: HashMap<char, i32> = HashMap::new();
        for c in s.chars() {
            let e = hm.entry(c).or_default();
            *e += 1;
        }
        for (i, c) in s.chars().enumerate() {
            if let Some(1) = hm.get(&c) {
                return i as i32;
            }
        }
        -1
    }
}

#[test]
fn test() {
    assert_eq!(Solution::first_uniq_char("leetcode".to_string()), 0);
    assert_eq!(Solution::first_uniq_char("loveleetcode".to_string()), 2);
}
