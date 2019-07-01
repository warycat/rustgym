struct Solution {}

impl Solution {
    fn reverse(x: i32) -> i32 {
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
            abs = abs / 10;
        }
        y * sign
    }
}

#[test]
fn overflow() {
    assert_eq!(Solution::reverse(2147483647), 0);
}

#[test]
fn pass() {
    assert_eq!(Solution::reverse(123456789), 987654321);
}

#[test]
fn negative() {
    assert_eq!(Solution::reverse(-123), -321);
}
