struct Solution;
use std::collections::VecDeque;

impl Solution {
    fn max_result(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let k = k as usize;
        let mut queue: VecDeque<(usize, i32)> = VecDeque::new();
        queue.push_back((0, nums[0]));
        for i in 1..n {
            while let Some((j, _)) = queue.front() {
                if j + k < i {
                    queue.pop_front();
                } else {
                    break;
                }
            }
            let max = queue.front().as_ref().unwrap().1 + nums[i];
            while let Some(&(_, sum)) = queue.back() {
                if sum < max {
                    queue.pop_back();
                } else {
                    break;
                }
            }
            queue.push_back((i, max));
        }
        queue.back().as_ref().unwrap().1
    }
}

#[test]
fn test() {
    let nums = vec![1, -1, -2, 4, -7, 3];
    let k = 2;
    let res = 7;
    assert_eq!(Solution::max_result(nums, k), res);
    let nums = vec![10, -5, -2, 4, 0, 3];
    let k = 3;
    let res = 17;
    assert_eq!(Solution::max_result(nums, k), res);
    let nums = vec![1, -5, -20, 4, -1, 3, -6, -3];
    let k = 2;
    let res = 0;
    assert_eq!(Solution::max_result(nums, k), res);
}
