struct Solution;

impl Solution {
    fn find_derangement(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![0; n + 1];
        dp[0] = 1;
        dp[1] = 0;
        for i in 2..=n {
            dp[i] = (i - 1) * (dp[i - 1] + dp[i - 2]) % 1_000_000_007;
        }
        dp[n] as i32
    }
}

#[test]
fn test() {
    let n = 3;
    let res = 2;
    assert_eq!(Solution::find_derangement(n), res);
}
