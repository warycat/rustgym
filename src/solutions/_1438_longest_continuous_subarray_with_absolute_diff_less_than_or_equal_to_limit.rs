struct Solution;
use std::collections::VecDeque;

impl Solution {
    fn longest_subarray(nums: Vec<i32>, limit: i32) -> i32 {
        let mut max_queue = VecDeque::new();
        let mut min_queue = VecDeque::new();
        let n = nums.len();
        let mut res = 0;
        let mut l = 0;
        for r in 0..n {
            while let Some(&last) = max_queue.back() {
                if last < nums[r] {
                    max_queue.pop_back();
                } else {
                    break;
                }
            }
            while let Some(&last) = min_queue.back() {
                if last > nums[r] {
                    min_queue.pop_back();
                } else {
                    break;
                }
            }
            max_queue.push_back(nums[r]);
            min_queue.push_back(nums[r]);
            if max_queue.front().unwrap() - min_queue.front().unwrap() > limit {
                if *max_queue.front().unwrap() == nums[l] {
                    max_queue.pop_front();
                }
                if *min_queue.front().unwrap() == nums[l] {
                    min_queue.pop_front();
                }
                l += 1;
            }
            res = res.max(r - l + 1);
        }
        res as i32
    }
}

#[test]
fn test() {
    let nums = vec![8, 2, 4, 7];
    let limit = 4;
    let res = 2;
    assert_eq!(Solution::longest_subarray(nums, limit), res);
}
