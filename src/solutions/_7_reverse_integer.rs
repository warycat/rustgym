pub struct Solution;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut abs = x.abs();
        let sign = x.signum();
        let mut y: i32 = 0;
        while abs > 0 {
            if let Some(v) = y.checked_mul(10) {
                y = v;
            } else {
                return 0;
            }
            if let Some(v) = y.checked_add(abs % 10) {
                y = v;
            } else {
                return 0;
            }
            abs /= 10;
        }
        y * sign
    }
}

#[test]
pub fn overflow() {
    assert_eq!(Solution::reverse(2_147_483_647), 0);
}

#[test]
pub fn pass() {
    assert_eq!(Solution::reverse(123_456_789), 987_654_321);
}

#[test]
pub fn negative() {
    assert_eq!(Solution::reverse(-123), -321);
}
