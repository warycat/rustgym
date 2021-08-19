struct Solution;

use std::collections::HashSet;

impl Solution {
    fn maximum_unique_subarray(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut sum = 0;
        let mut res = 0;
        let mut hs: HashSet<i32> = HashSet::new();
        let mut left = 0;
        for right in 0..n {
            if hs.insert(nums[right]) {
                sum += nums[right];
            } else {
                while nums[left] != nums[right] {
                    hs.remove(&nums[left]);
                    sum -= nums[left];
                    left += 1;
                }
                left += 1;
            }
            res = res.max(sum);
        }
        res
    }
}

#[test]
fn test() {
    let nums = vec![4, 2, 4, 5, 6];
    let res = 17;
    assert_eq!(Solution::maximum_unique_subarray(nums), res);
    let nums = vec![5, 2, 1, 2, 5, 2, 1, 2, 5];
    let res = 8;
    assert_eq!(Solution::maximum_unique_subarray(nums), res);
}
