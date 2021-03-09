struct Solution;

impl Solution {
    fn max_sum_after_operation(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut dp: Vec<[i32; 2]> = vec![[0, 0]; n];
        dp[0][0] = nums[0];
        dp[0][1] = nums[0] * nums[0];
        let mut res = dp[0][1];
        for i in 1..n {
            dp[i][0] = nums[i].max(dp[i - 1][0] + nums[i]);
            dp[i][1] = (nums[i] * nums[i])
                .max(dp[i - 1][0] + nums[i] * nums[i])
                .max(dp[i - 1][1] + nums[i]);
            res = res.max(dp[i][1]);
        }
        res
    }
}

#[test]
fn test() {
    let nums = vec![2, -1, -4, -3];
    let res = 17;
    assert_eq!(Solution::max_sum_after_operation(nums), res);
    let nums = vec![2, -1, -4, -3];
    let res = 17;
    assert_eq!(Solution::max_sum_after_operation(nums), res);
}
