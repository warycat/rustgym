struct Solution;
use std::collections::HashSet;

impl Solution {
    fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
        let n = nums.len();
        let mut sum = 0;
        let mut pre = 0;
        let mut hs: HashSet<i32> = HashSet::new();
        for i in 0..n {
            sum += nums[i];
            let cur = if k == 0 { sum } else { sum % k };
            if hs.contains(&cur) {
                return true;
            }
            hs.insert(pre);
            pre = cur;
        }
        false
    }
}

#[test]
fn test() {
    let nums = vec![23, 2, 4, 6, 7];
    let k = 6;
    let res = true;
    assert_eq!(Solution::check_subarray_sum(nums, k), res);
    let nums = vec![23, 2, 6, 4, 7];
    let k = 6;
    let res = true;
    assert_eq!(Solution::check_subarray_sum(nums, k), res);
    let nums = vec![23, 2, 6, 4, 7];
    let k = 0;
    let res = false;
    assert_eq!(Solution::check_subarray_sum(nums, k), res);
    let nums = vec![0, 0];
    let k = -1;
    let res = true;
    assert_eq!(Solution::check_subarray_sum(nums, k), res);
}
