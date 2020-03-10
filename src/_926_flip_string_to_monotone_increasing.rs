struct Solution;

impl Solution {
    fn min_flips_mono_incr(s: String) -> i32 {
        let n = s.len();
        let s: Vec<char> = s.chars().collect();
        let mut left = vec![0; n];
        let mut ones = 0;
        let mut right = vec![0; n];
        let mut zeros = 0;
        for i in 0..n {
            left[i] = ones;
            if s[i] == '1' {
                ones += 1;
            }
        }
        for i in (0..n).rev() {
            right[i] = zeros;
            if s[i] == '0' {
                zeros += 1;
            }
        }
        let mut res = std::i32::MAX;
        for i in 0..n {
            res = res.min(left[i] + right[i]);
        }
        res
    }
}

#[test]
fn test() {
    let s = "00110".to_string();
    let res = 1;
    assert_eq!(Solution::min_flips_mono_incr(s), res);
    let s = "010110".to_string();
    let res = 2;
    assert_eq!(Solution::min_flips_mono_incr(s), res);
    let s = "00011000".to_string();
    let res = 2;
    assert_eq!(Solution::min_flips_mono_incr(s), res);
}
