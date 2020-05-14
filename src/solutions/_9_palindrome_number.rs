pub struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        let mut y = 0;
        let mut z = x;
        while z > 0 {
            y *= 10;
            y += z % 10;
            z /= 10;
        }
        x == y
    }
}

#[test]
pub fn negative() {
    assert_eq!(Solution::is_palindrome(-123), false);
}

#[test]
pub fn yes() {
    assert_eq!(Solution::is_palindrome(12321), true);
}
