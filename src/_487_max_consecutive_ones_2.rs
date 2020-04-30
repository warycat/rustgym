struct Solution;
use std::collections::VecDeque;

impl Solution {
    fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut res = 0;
        let mut queue: VecDeque<usize> = VecDeque::from(vec![0, 0]);
        for i in 0..n {
            *queue.back_mut().unwrap() += 1;
            if nums[i] == 0 {
                queue.pop_front();
                queue.push_back(0);
            }
            res = res.max(queue.iter().sum());
        }
        res as i32
    }
}

#[test]
fn test() {
    let nums = vec![1, 0, 1, 1, 0];
    let res = 4;
    assert_eq!(Solution::find_max_consecutive_ones(nums), res);
    let nums = vec![1, 1, 0, 1];
    let res = 4;
    assert_eq!(Solution::find_max_consecutive_ones(nums), res);
}
