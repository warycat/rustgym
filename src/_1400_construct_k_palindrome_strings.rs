struct Solution;

impl Solution {
    fn can_construct(s: String, k: i32) -> bool {
        let k = k as usize;
        let n = s.len();
        if n < k {
            return false;
        }
        let mut count = vec![0; 26];
        for c in s.bytes() {
            count[(c - b'a') as usize] += 1;
        }
        let mut odd = 0;
        for v in count {
            if v % 2 != 0 {
                odd += 1;
            }
        }
        odd <= k
    }
}

#[test]
fn test() {
    let s = "annabelle".to_string();
    let k = 2;
    let res = true;
    assert_eq!(Solution::can_construct(s, k), res);
    let s = "leetcode".to_string();
    let k = 3;
    let res = false;
    assert_eq!(Solution::can_construct(s, k), res);
    let s = "true".to_string();
    let k = 4;
    let res = true;
    assert_eq!(Solution::can_construct(s, k), res);
    let s = "yzyzyzyzyzyzyzy".to_string();
    let k = 2;
    let res = true;
    assert_eq!(Solution::can_construct(s, k), res);
    let s = "cr".to_string();
    let k = 7;
    let res = false;
    assert_eq!(Solution::can_construct(s, k), res);
    let s = "aaa".to_string();
    let k = 2;
    let res = true;
    assert_eq!(Solution::can_construct(s, k), res);
}
