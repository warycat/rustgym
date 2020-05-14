struct Solution;

impl Solution {
    fn find_length(a: Vec<i32>, b: Vec<i32>) -> i32 {
        let n = a.len();
        let m = b.len();
        let mut dp: Vec<Vec<i32>> = vec![vec![0; m + 1]; n + 1];
        let mut res = 0;
        for i in 0..n {
            for j in 0..m {
                if a[i] == b[j] {
                    dp[i + 1][j + 1] = dp[i][j] + 1;
                    res = res.max(dp[i + 1][j + 1]);
                }
            }
        }
        res
    }
}

#[test]
fn test() {
    let a = vec![1, 2, 3, 2, 1];
    let b = vec![3, 2, 1, 4, 7];
    let res = 3;
    assert_eq!(Solution::find_length(a, b), res);
}
