struct Solution;

impl Solution {
    fn trailing_zeroes(mut n: i32) -> i32 {
        let mut sum = 0;
        while n > 0 {
            sum += n / 5;
            n /= 5;
        }
        sum
    }
}

#[test]
fn test() {
    assert_eq!(Solution::trailing_zeroes(3), 0);
    assert_eq!(Solution::trailing_zeroes(5), 1);
    assert_eq!(Solution::trailing_zeroes(10), 2);
}
