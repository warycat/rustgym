struct Solution;

use std::collections::HashMap;

impl Solution {
    fn find_target_sum_ways(nums: Vec<i32>, s: i32) -> i32 {
        let n = nums.len();
        let mut memo: HashMap<(usize, i32), i32> = HashMap::new();
        Self::dfs(0, s, &mut memo, &nums, n)
    }

    fn dfs(
        start: usize,
        sum: i32,
        memo: &mut HashMap<(usize, i32), i32>,
        nums: &[i32],
        n: usize,
    ) -> i32 {
        if start == n {
            if sum == 0 {
                1
            } else {
                0
            }
        } else {
            if let Some(&res) = memo.get(&(start, sum)) {
                res
            } else {
                let a = Self::dfs(start + 1, sum + nums[start], memo, nums, n);
                let b = Self::dfs(start + 1, sum - nums[start], memo, nums, n);
                let res = a + b;
                memo.insert((start, sum), res);
                res
            }
        }
    }
}

#[test]
fn test() {
    let nums = vec![1, 1, 1, 1, 1];
    let s = 3;
    let res = 5;
    assert_eq!(Solution::find_target_sum_ways(nums, s), res);
}
