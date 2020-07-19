struct Solution;

use std::cmp::Reverse;

impl Solution {
    fn make_largest_special(s: String) -> String {
        let mut count = 0;
        let mut v = vec![];
        let mut ss = "".to_string();
        for c in s.chars() {
            ss.push(c);
            if c == '1' {
                count += 1;
            } else {
                count -= 1;
            }
            if count == 0 {
                let n = ss.len();
                let t = format!("1{}0", Self::make_largest_special(ss[1..n - 1].to_string()));
                v.push(t);
                ss = "".to_string();
            }
        }
        v.sort_unstable_by_key(|s| Reverse(s.to_string()));
        v.join("")
    }
}

#[test]
fn test() {
    let s = "11011000".to_string();
    let res = "11100100".to_string();
    assert_eq!(Solution::make_largest_special(s), res);
}
