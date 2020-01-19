struct Solution;

impl Solution {
    fn my_pow(x: f64, n: i32) -> f64 {
        if n == 0 {
            1.0
        } else if n % 2 == 0 {
            Self::my_pow(x * x, n / 2)
        } else if n < 0 {
            1.0 / Self::my_pow(x, -n)
        } else {
            x * Self::my_pow(x * x, n / 2)
        }
    }
}

#[test]
fn test() {
    assert_eq!(Solution::my_pow(2.0, 10), 1024.0);
    assert_eq!(Solution::my_pow(2.0, -3), 0.125);
    assert_eq!(Solution::my_pow(1.0, -2147483648), 1.0);
}
