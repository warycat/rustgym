pub struct Solution;

use std::cmp::max;
use std::i32::MIN;

impl Solution {
    fn max_sub_array(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut sums = vec![0; n];
        let mut res = MIN;
        for i in 0..n {
            if i == 0 {
                sums[0] = nums[0];
            } else {
                sums[i] = max(nums[i], sums[i - 1] + nums[i])
            }
            res = max(res, sums[i])
        }
        res
    }
}

#[test]
fn test() {
    let nums = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
    assert_eq!(Solution::max_sub_array(nums), 6);
}

#[test]
fn test1() {
    let nums = vec![-1];
    assert_eq!(Solution::max_sub_array(nums), -1);
}
