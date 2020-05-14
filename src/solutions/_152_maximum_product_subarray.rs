struct Solution;

use std::i32;
use std::mem::swap;

impl Solution {
    fn max_product(nums: Vec<i32>) -> i32 {
        let mut res = nums[0];
        let mut min = res;
        let mut max = res;
        let n = nums.len();
        for i in 1..n {
            let x = nums[i];
            if x < 0 {
                swap(&mut min, &mut max);
            }
            max = i32::max(x, max * x);
            min = i32::min(x, min * x);
            res = i32::max(res, max);
        }
        res
    }
}

#[test]
fn test() {
    let nums = vec![2, 3, -2, 4];
    assert_eq!(Solution::max_product(nums), 6);
    let nums = vec![-2, 0, -1];
    assert_eq!(Solution::max_product(nums), 0);
}
