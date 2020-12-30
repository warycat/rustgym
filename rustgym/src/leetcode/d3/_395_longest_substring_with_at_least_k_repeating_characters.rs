struct Solution;
use std::collections::HashMap;

impl Solution {
    fn longest_substring(s: String, k: i32) -> i32 {
        let mut hm: HashMap<char, usize> = HashMap::new();
        for c in s.chars() {
            *hm.entry(c).or_default() += 1;
        }
        for (c, v) in hm {
            if v < k as usize {
                return s
                    .split_terminator(c)
                    .map(|s| Self::longest_substring(s.to_string(), k))
                    .max()
                    .unwrap();
            }
        }
        s.len() as i32
    }
}

#[test]
fn test() {
    let s = "aaabb".to_string();
    let k = 3;
    let res = 3;
    assert_eq!(Solution::longest_substring(s, k), res);
    let s = "ababbc".to_string();
    let k = 2;
    let res = 5;
    assert_eq!(Solution::longest_substring(s, k), res);
    let s = "ababbc".to_string();
    let k = 3;
    let res = 0;
    assert_eq!(Solution::longest_substring(s, k), res);
}
