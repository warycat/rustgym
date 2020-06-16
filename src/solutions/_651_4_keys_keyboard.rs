struct Solution;

impl Solution {
    fn max_a(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![0; n + 1];
        for i in 0..=n {
            dp[i] = i as i32;
            if i > 2 {
                for j in 1..i - 2 {
                    dp[i] = dp[i].max(dp[j] * (i - j - 1) as i32);
                }
            }
        }
        dp[n]
    }
}

#[test]
fn test() {
    let n = 3;
    let res = 3;
    assert_eq!(Solution::max_a(n), res);
    let n = 7;
    let res = 9;
    assert_eq!(Solution::max_a(n), res);
}
