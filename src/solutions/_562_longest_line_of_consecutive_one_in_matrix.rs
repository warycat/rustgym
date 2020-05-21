struct Solution;

impl Solution {
    fn longest_line(matrix: Vec<Vec<i32>>) -> i32 {
        let n = matrix.len();
        if n == 0 {
            return 0;
        }
        let m = matrix[0].len();
        let mut dp: Vec<Vec<Vec<i32>>> = vec![vec![vec![0; 4]; m]; n];
        let mut res = 0;
        for i in 0..n {
            for j in 0..m {
                if matrix[i][j] == 1 {
                    dp[i][j][0] += 1;
                    dp[i][j][1] += 1;
                    dp[i][j][2] += 1;
                    dp[i][j][3] += 1;
                    if i > 0 {
                        dp[i][j][0] += dp[i - 1][j][0];
                    }
                    if j > 0 {
                        dp[i][j][1] += dp[i][j - 1][1];
                    }
                    if i > 0 && j > 0 {
                        dp[i][j][2] += dp[i - 1][j - 1][2];
                    }
                    if i > 0 && j + 1 < m {
                        dp[i][j][3] += dp[i - 1][j + 1][3];
                    }
                    res = res.max(*dp[i][j].iter().max().unwrap());
                }
            }
        }
        res
    }
}

#[test]
fn test() {
    let matrix = vec_vec_i32![[0, 1, 1, 0], [0, 1, 1, 0], [0, 0, 0, 1]];
    let res = 3;
    assert_eq!(Solution::longest_line(matrix), res);
}
