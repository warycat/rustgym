struct Solution;

use std::collections::HashMap;

impl Solution {
    fn max_length_between_equal_characters(s: String) -> i32 {
        let mut hm: HashMap<char, usize> = HashMap::new();
        let mut max = std::usize::MIN;
        for (i, c) in s.char_indices() {
            if let Some(j) = hm.get(&c) {
                max = max.max(i - j);
            }
            hm.entry(c).or_insert(i);
        }
        if max > 0 {
            max as i32 - 1
        } else {
            -1
        }
    }
}

#[test]
fn test() {
    let s = "aa".to_string();
    let res = 0;
    assert_eq!(Solution::max_length_between_equal_characters(s), res);
    let s = "abca".to_string();
    let res = 2;
    assert_eq!(Solution::max_length_between_equal_characters(s), res);
    let s = "cbzxy".to_string();
    let res = -1;
    assert_eq!(Solution::max_length_between_equal_characters(s), res);
    let s = "cabbac".to_string();
    let res = 4;
    assert_eq!(Solution::max_length_between_equal_characters(s), res);
    let s = "mgntdygtxrvxjnwksqhxuxtrv".to_string();
    let res = 18;
    assert_eq!(Solution::max_length_between_equal_characters(s), res);
}
