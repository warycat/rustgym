struct Solution;

impl Solution {
    fn probability_of_heads(prob: Vec<f64>, target: i32) -> f64 {
        let n = prob.len();
        let mut dp = vec![vec![0.0; n + 1]; n + 1];
        dp[0][0] = 1.0;
        for i in 0..n {
            for j in 0..=i {
                dp[i + 1][j + 1] += dp[i][j] * prob[i];
                dp[i + 1][j] += dp[i][j] * (1.0 - prob[i]);
            }
        }
        dp[n][target as usize]
    }
}

#[test]
fn test() {
    use assert_approx_eq::assert_approx_eq;
    let prob = vec![0.4];
    let target = 1;
    let res = 0.4;
    assert_approx_eq!(Solution::probability_of_heads(prob, target), res);
    let prob = vec![0.5, 0.5, 0.5, 0.5, 0.5];
    let target = 0;
    let res = 0.03125;
    assert_approx_eq!(Solution::probability_of_heads(prob, target), res);
}
