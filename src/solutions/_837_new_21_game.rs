struct Solution;

impl Solution {
    fn new21_game(n: i32, k: i32, w: i32) -> f64 {
        if k == 0 || n > k + w {
            return 1.0;
        }
        let n = n as usize;
        let w = w as usize;
        let k = k as usize;
        let mut dp: Vec<f64> = vec![0.0; n + 1];
        dp[0] = 1.0;
        let mut sum = 1.0;
        let mut res = 0.0;
        for i in 1..=n {
            dp[i] = sum / w as f64;
            if i < k {
                sum += dp[i];
            } else {
                res += dp[i];
            }
            if i >= w {
                sum -= dp[i - w];
            }
        }
        res
    }
}

#[test]
fn test() {
    use assert_approx_eq::assert_approx_eq;
    let n = 10;
    let k = 1;
    let w = 10;
    let res = 1.0;
    assert_approx_eq!(Solution::new21_game(n, k, w), res);
    let n = 6;
    let k = 1;
    let w = 10;
    let res = 0.6;
    assert_approx_eq!(Solution::new21_game(n, k, w), res);
    let n = 21;
    let k = 17;
    let w = 10;
    let res = 0.732777;
    assert_approx_eq!(Solution::new21_game(n, k, w), res);
}
