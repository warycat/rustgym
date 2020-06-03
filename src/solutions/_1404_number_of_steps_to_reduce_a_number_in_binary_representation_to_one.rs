struct Solution;

impl Solution {
    fn num_steps(s: String) -> i32 {
        let mut carry = 0;
        let mut res = 0;
        let n = s.len();
        let s: Vec<u8> = s.bytes().collect();
        for i in (1..n).rev() {
            res += 1;
            if s[i] - b'0' + carry == 1 {
                carry = 1;
                res += 1;
            }
        }
        res + carry as i32
    }
}

#[test]
fn test() {
    let s = "1101".to_string();
    let res = 6;
    assert_eq!(Solution::num_steps(s), res);
    let s = "10".to_string();
    let res = 1;
    assert_eq!(Solution::num_steps(s), res);
    let s = "1".to_string();
    let res = 0;
    assert_eq!(Solution::num_steps(s), res);
}
