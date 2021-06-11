struct Solution;

impl Solution {
    fn check_zero_ones(s: String) -> bool {
        let s: Vec<char> = s.chars().collect();
        let n = s.len();
        let mut state = '2';
        let mut length = 0;
        let mut zero = 0;
        let mut one = 0;
        for i in 0..n {
            if s[i] == state {
                length += 1;
            } else {
                state = s[i];
                length = 1;
            }
            if s[i] == '0' {
                zero = zero.max(length);
            } else {
                one = one.max(length);
            }
        }
        one > zero
    }
}

#[test]
fn test() {
    let s = "1101".to_string();
    let res = true;
    assert_eq!(Solution::check_zero_ones(s), res);
    let s = "111000".to_string();
    let res = false;
    assert_eq!(Solution::check_zero_ones(s), res);
    let s = "110100010".to_string();
    let res = false;
    assert_eq!(Solution::check_zero_ones(s), res);
}
