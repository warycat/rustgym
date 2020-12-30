struct Solution;

impl Solution {
    fn num_squares(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![std::usize::MAX; n + 1];
        dp[0] = 0;
        for i in 1..=n {
            let mut j = 1;
            while j * j <= i {
                dp[i] = dp[i].min(dp[i - j * j] + 1);
                j += 1;
            }
        }
        dp[n] as i32
    }
}

#[test]
fn test() {
    let n = 12;
    let res = 3;
    assert_eq!(Solution::num_squares(n), res);
    let n = 13;
    let res = 2;
    assert_eq!(Solution::num_squares(n), res);
}
