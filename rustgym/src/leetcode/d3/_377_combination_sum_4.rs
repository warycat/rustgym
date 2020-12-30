struct Solution;

impl Solution {
    fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
        let k = target as usize;
        let mut dp = vec![0; k + 1];
        dp[0] = 1;
        for i in 1..=target {
            for &j in &nums {
                if i - j >= 0 {
                    dp[i as usize] += dp[(i - j) as usize];
                }
            }
        }
        dp[k]
    }
}

#[test]
fn test() {
    let nums = vec![1, 2, 3];
    let target = 4;
    let res = 7;
    assert_eq!(Solution::combination_sum4(nums, target), res);
}
