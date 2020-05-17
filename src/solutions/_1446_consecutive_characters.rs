struct Solution;

impl Solution {
    fn max_power(s: String) -> i32 {
        let mut prev: (char, usize) = (' ', 0);
        let mut res = 0;
        for c in s.chars() {
            if c == prev.0 {
                prev.1 += 1;
            } else {
                prev = (c, 1);
            }
            res = res.max(prev.1);
        }
        res as i32
    }
}

#[test]
fn test() {
    let s = "leetcode".to_string();
    let res = 2;
    assert_eq!(Solution::max_power(s), res);
    let s = "abbcccddddeeeeedcba".to_string();
    let res = 5;
    assert_eq!(Solution::max_power(s), res);
    let s = "triplepillooooow".to_string();
    let res = 5;
    assert_eq!(Solution::max_power(s), res);
    let s = "hooraaaaaaaaaaay".to_string();
    let res = 11;
    assert_eq!(Solution::max_power(s), res);
    let s = "tourist".to_string();
    let res = 1;
    assert_eq!(Solution::max_power(s), res);
}
