struct Solution;
use std::cmp::Ordering::*;

impl Solution {
    fn rob(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut max = vec![0; n];
        for i in 0..n {
            match i.cmp(&2) {
                Less => {
                    max[i] = nums[i];
                }
                Equal => {
                    max[2] = nums[2] + nums[0];
                }
                Greater => {
                    max[i] = nums[i] + i32::max(max[i - 2], max[i - 3]);
                }
            }
        }
        let mut res = 0;
        for i in 0..n {
            res = i32::max(max[i], res);
        }
        res
    }
}

#[test]
fn test() {
    let nums = vec![1, 2, 3, 1];
    assert_eq!(Solution::rob(nums), 4);
    let nums = vec![2, 7, 9, 3, 1];
    assert_eq!(Solution::rob(nums), 12);
}
