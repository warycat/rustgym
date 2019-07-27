struct Solution;

use std::i32;

impl Solution {
    fn is_power_of_four(mut n: i32) -> bool {
        while n != 0 && n % 4 == 0 {
            n /= 4;
        }
        n == 1
    }
}

#[test]
fn test() {
    assert_eq!(Solution::is_power_of_four(16), true);
}
