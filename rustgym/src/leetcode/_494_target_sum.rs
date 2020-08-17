struct Solution;

use std::collections::HashMap;

impl Solution {
    fn find_target_sum_ways(nums: Vec<i32>, s: i32) -> i32 {
        let n = nums.len();
        let mut memo: HashMap<(usize, i32), i32> = HashMap::new();
        Self::dp(n, s, &mut memo, &nums, n)
    }

    fn dp(
        end: usize,
        sum: i32,
        memo: &mut HashMap<(usize, i32), i32>,
        nums: &[i32],
        n: usize,
    ) -> i32 {
        if end == 0 {
            if sum == 0 {
                1
            } else {
                0
            }
        } else {
            if let Some(&res) = memo.get(&(end, sum)) {
                return res;
            }
            let a = Self::dp(end - 1, sum + nums[end - 1], memo, nums, n);
            let b = Self::dp(end - 1, sum - nums[end - 1], memo, nums, n);
            let res = a + b;
            memo.insert((end, sum), res);
            res
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
