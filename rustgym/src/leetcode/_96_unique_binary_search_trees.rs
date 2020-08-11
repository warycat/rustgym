struct Solution;

impl Solution {
    fn num_trees(n: i32) -> i32 {
        let n = n as usize;
        let mut dp: Vec<i32> = vec![0; n + 1];
        dp[0] = 1;
        dp[1] = 1;
        for i in 2..=n {
            for j in 0..i {
                dp[i] += dp[j] * dp[i - 1 - j];
            }
        }
        dp[n]
    }
}

#[test]
fn test() {
    let n = 3;
    let res = 5;
    assert_eq!(Solution::num_trees(n), res);
}
