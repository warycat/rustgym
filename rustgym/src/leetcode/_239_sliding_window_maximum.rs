struct Solution;
use std::collections::VecDeque;

impl Solution {
    fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let n = nums.len();
        let mut queue: VecDeque<usize> = VecDeque::new();
        let mut res = vec![];
        for i in 0..n {
            let n = queue.len();
            for _ in 0..n {
                let j = queue.pop_front().unwrap();
                if i - j < k && nums[j] >= nums[i] {
                    queue.push_back(j);
                }
            }
            queue.push_back(i);
            if i + 1 >= k {
                res.push(nums[*queue.front().unwrap()]);
            }
        }
        res
    }
}

#[test]
fn test() {
    let nums = vec![1, 3, -1, -3, 5, 3, 6, 7];
    let k = 3;
    let res = vec![3, 3, 5, 5, 6, 7];
    assert_eq!(Solution::max_sliding_window(nums, k), res);
}
