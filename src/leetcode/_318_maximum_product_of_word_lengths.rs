struct Solution;
use std::collections::HashMap;

impl Solution {
    fn max_product(words: Vec<String>) -> i32 {
        let mut hm: HashMap<u32, usize> = HashMap::new();
        for word in words {
            let mut mask: u32 = 0;
            for c in word.bytes() {
                mask |= 1 << (c - b'a');
            }
            let size = hm.entry(mask).or_default();
            *size = word.len().max(*size);
        }
        let mut res = 0;
        for (&ka, &va) in &hm {
            for (&kb, &vb) in &hm {
                if ka & kb == 0 {
                    res = res.max(va * vb);
                }
            }
        }
        res as i32
    }
}

#[test]
fn test() {
    let words = vec_string!["abcw", "baz", "foo", "bar", "xtfn", "abcdef"];
    let res = 16;
    assert_eq!(Solution::max_product(words), res);
    let words = vec_string!["a", "ab", "abc", "d", "cd", "bcd", "abcd"];
    let res = 4;
    assert_eq!(Solution::max_product(words), res);
    let words = vec_string!["a", "aa", "aaa", "aaaa"];
    let res = 0;
    assert_eq!(Solution::max_product(words), res);
}
