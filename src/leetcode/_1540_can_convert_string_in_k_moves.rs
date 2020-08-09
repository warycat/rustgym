struct Solution;

use std::collections::HashMap;

impl Solution {
    fn can_convert_string(s: String, t: String, k: i32) -> bool {
        let n = s.len();
        let m = t.len();
        if n != m {
            return false;
        }
        let s: Vec<i32> = s.bytes().map(|b| (b - b'a') as i32).collect();
        let t: Vec<i32> = t.bytes().map(|b| (b - b'a') as i32).collect();
        let mut count: HashMap<i32, i32> = HashMap::new();
        for i in 0..n {
            if s[i] == t[i] {
                continue;
            }
            *count.entry((26 + t[i] - s[i]) % 26).or_default() += 1;
        }
        let mut max = 0;
        for (k, v) in count {
            max = max.max((v - 1) * 26 + k);
        }
        max <= k
    }
}

#[test]
fn test() {
    let s = "input".to_string();
    let t = "ouput".to_string();
    let k = 9;
    let res = true;
    assert_eq!(Solution::can_convert_string(s, t, k), res);
    let s = "abc".to_string();
    let t = "bcd".to_string();
    let k = 10;
    let res = false;
    assert_eq!(Solution::can_convert_string(s, t, k), res);
    let s = "aab".to_string();
    let t = "bbb".to_string();
    let k = 27;
    let res = true;
    assert_eq!(Solution::can_convert_string(s, t, k), res);
}
