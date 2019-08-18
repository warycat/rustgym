struct Solution;

impl Solution {
    fn is_power_of_two(n: i32) -> bool {
        if n <= 0 {
            return false;
        }
        n & (n - 1) == 0
    }
}

#[test]
fn test() {
    assert_eq!(Solution::is_power_of_two(1), true);
    assert_eq!(Solution::is_power_of_two(16), true);
    assert_eq!(Solution::is_power_of_two(218), false);
}
