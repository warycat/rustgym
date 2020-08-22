struct Solution;

use std::collections::HashMap;

impl Solution {
    fn longest_awesome(s: String) -> i32 {
        let mut mask = 0;
        let mut hm: HashMap<u32, usize> = HashMap::new();
        hm.insert(0, 0);
        let mut res = 0;
        for (i, b) in s.bytes().enumerate() {
            mask ^= 1 << (b - b'0');
            if let Some(j) = hm.get(&mask) {
                res = res.max(i + 1 - j);
            }
            for k in 0..10 {
                if let Some(j) = hm.get(&(mask ^ (1 << k))) {
                    res = res.max(i + 1 - j);
                }
            }
            hm.entry(mask).or_insert(i + 1);
        }
        res as i32
    }
}

#[test]
fn test() {
    let s = "3242415".to_string();
    let res = 5;
    assert_eq!(Solution::longest_awesome(s), res);
    let s = "12345678".to_string();
    let res = 1;
    assert_eq!(Solution::longest_awesome(s), res);
    let s = "213123".to_string();
    let res = 6;
    assert_eq!(Solution::longest_awesome(s), res);
    let s = "00".to_string();
    let res = 2;
    assert_eq!(Solution::longest_awesome(s), res);
}
