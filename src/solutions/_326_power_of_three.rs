struct Solution;

impl Solution {
    fn is_power_of_three(n: i32) -> bool {
        n > 0 && 1_162_261_467 % n == 0
    }
}

#[test]
fn test() {
    assert_eq!(Solution::is_power_of_three(27), true);
    assert_eq!(Solution::is_power_of_three(0), false);
    assert_eq!(Solution::is_power_of_three(9), true);
    assert_eq!(Solution::is_power_of_three(45), false);
}
