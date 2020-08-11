struct Solution;

use std::collections::HashMap;

impl Solution {
    fn find_the_difference(s: String, t: String) -> char {
        let mut hm: HashMap<char, i32> = HashMap::new();
        for c in t.chars() {
            let e = hm.entry(c).or_default();
            *e += 1;
        }
        for c in s.chars() {
            let e = hm.entry(c).or_default();
            *e -= 1;
        }
        for (&c, &v) in hm.iter() {
            if v == 1 {
                return c;
            }
        }
        unreachable!()
    }
}

#[test]
fn test() {
    let s = "abcd".to_string();
    let t = "abcde".to_string();
    let c = 'e';
    assert_eq!(Solution::find_the_difference(s, t), c);
}
