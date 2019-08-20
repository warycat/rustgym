struct Solution;

use std::i32;

impl Solution {
    fn find_unsorted_subarray(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut min = i32::MAX;
        let mut max = i32::MIN;
        let mut l = n - 1;
        let mut r = 0;

        for i in 0..n {
            max = i32::max(max, nums[i]);
            if max != nums[i] {
                r = i;
            }
        }

        for i in (0..n).rev() {
            min = i32::min(min, nums[i]);
            if min != nums[i] {
                l = i;
            }
        }
        if r <= l {
            0
        } else {
            (r - l + 1) as i32
        }
    }
}

#[test]
fn test() {
    let nums = vec![2, 6, 4, 8, 10, 9, 15];
    assert_eq!(Solution::find_unsorted_subarray(nums), 5);
    let nums = vec![1, 2, 3, 4, 5];
    assert_eq!(Solution::find_unsorted_subarray(nums), 0);
}
