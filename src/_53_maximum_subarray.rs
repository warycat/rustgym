struct Solution;

use std::i32::MIN;

impl Solution {
    fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut prev: Option<i32> = None;
        let mut res = MIN;
        for x in nums {
            if let Some(sum) = prev {
                prev = Some(i32::max(x, sum + x));
            } else {
                prev = Some(x);
            }
            res = i32::max(res, prev.unwrap());
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
