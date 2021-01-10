struct Solution;

impl Solution {
    fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        let mut reversed = 0;
        let mut number = x;
        while number > 0 {
            reversed = reversed * 10 + number % 10;
            number /= 10;
        }
        x == reversed
        // or compare original number as a string with the reversed string
        // let x_str = x.abs().to_string();
        // x >= 0 && x_str == x_str.chars().rev().collect::<String>()
    }
}

#[test]
fn test() {
    assert_eq!(Solution::is_palindrome(-123), false);
    assert_eq!(Solution::is_palindrome(12321), true);
}
