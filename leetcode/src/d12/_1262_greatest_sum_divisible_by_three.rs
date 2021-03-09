struct Solution;

impl Solution {
    fn max_sum_div_three(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut dp = vec![vec![0; 3]; 2];
        for i in 0..n {
            for j in 0..3 {
                dp[(i + 1) % 2][j] = dp[(i + 1) % 2][j].max(dp[i % 2][j]);
                let sum = dp[i % 2][j] + nums[i];
                let sum_mod_3 = (sum % 3) as usize;
                dp[(i + 1) % 2][sum_mod_3] = dp[(i + 1) % 2][sum_mod_3].max(sum);
            }
        }
        dp[n % 2][0]
    }
}

#[test]
fn test() {
    let nums = vec![3, 6, 5, 1, 8];
    let res = 18;
    assert_eq!(Solution::max_sum_div_three(nums), res);
    let nums = vec![4];
    let res = 0;
    assert_eq!(Solution::max_sum_div_three(nums), res);
    let nums = vec![1, 2, 3, 4, 4];
    let res = 12;
    assert_eq!(Solution::max_sum_div_three(nums), res);
}
