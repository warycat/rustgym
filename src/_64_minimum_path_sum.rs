struct Solution;

impl Solution {
    fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();
        let mut dp: Vec<Vec<i32>> = vec![vec![0; m]; n];
        dp[0][0] = grid[0][0];
        for j in 1..m {
            dp[0][j] = dp[0][j - 1] + grid[0][j];
        }
        for i in 1..n {
            dp[i][0] = dp[i - 1][0] + grid[i][0];
        }
        for i in 1..n {
            for j in 1..m {
                dp[i][j] = i32::min(dp[i - 1][j], dp[i][j - 1]) + grid[i][j];
            }
        }
        dp[n - 1][m - 1]
    }
}

#[test]
fn test() {
    let grid: Vec<Vec<i32>> = vec_vec_i32![[1, 3, 1], [1, 5, 1], [4, 2, 1]];
    let res = 7;
    assert_eq!(Solution::min_path_sum(grid), res);
}
