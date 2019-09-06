struct Solution;

use std::i32;

impl Solution {
    fn sum_of_digits(a: Vec<i32>) -> i32 {
        let mut min = i32::MAX;
        for x in a {
            min = i32::min(x, min);
        }
        let mut sum = 0;
        while min > 0 {
            sum += min % 10;
            min /= 10;
        }
        if sum % 2 == 0 {
            1
        } else {
            0
        }
    }
}

#[test]
fn test() {
    let a = vec![34, 23, 1, 24, 75, 33, 54, 8];
    assert_eq!(Solution::sum_of_digits(a), 0);
    let a = vec![99, 77, 33, 66, 55];
    assert_eq!(Solution::sum_of_digits(a), 1);
}
