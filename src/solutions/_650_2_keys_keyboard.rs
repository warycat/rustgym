struct Solution;

impl Solution {
    fn min_steps(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![0; n + 1];
        for i in 2..=n {
            dp[i] = i;
            for j in (2..i).rev() {
                if i % j == 0 {
                    dp[i] = dp[j] + i / j;
                    break;
                }
            }
        }
        dp[n] as i32
    }
}

#[test]
fn test() {
    let n = 3;
    let res = 3;
    assert_eq!(Solution::min_steps(n), res);
}
