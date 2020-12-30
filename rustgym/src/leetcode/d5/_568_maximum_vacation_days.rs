struct Solution;

impl Solution {
    fn max_vacation_days(flights: Vec<Vec<i32>>, days: Vec<Vec<i32>>) -> i32 {
        let n = days.len();
        let w = days[0].len();
        let mut dp = vec![vec![-1; n]; w];
        dp[0][0] = days[0][0];
        for i in 0..n {
            if flights[0][i] == 1 {
                dp[0][i] = days[i][0];
            }
        }
        let mut res = 0;

        for k in 1..w {
            for i in 0..n {
                for j in 0..n {
                    if (flights[j][i] == 1 || j == i) && dp[k - 1][j] != -1 {
                        dp[k][i] = dp[k][i].max(dp[k - 1][j] + days[i][k]);
                    }
                }
            }
        }
        for k in 0..w {
            for i in 0..n {
                res = res.max(dp[k][i]);
            }
        }
        res
    }
}

#[test]
fn test() {
    let flights = vec_vec_i32![[0, 1, 1], [1, 0, 1], [1, 1, 0]];
    let days = vec_vec_i32![[1, 3, 1], [6, 0, 3], [3, 3, 3]];
    let res = 12;
    assert_eq!(Solution::max_vacation_days(flights, days), res);
    let flights = vec_vec_i32![[0, 0, 0], [0, 0, 0], [0, 0, 0]];
    let days = vec_vec_i32![[1, 1, 1], [7, 7, 7], [7, 7, 7]];
    let res = 3;
    assert_eq!(Solution::max_vacation_days(flights, days), res);
    let flights = vec_vec_i32![[0, 1, 1], [1, 0, 1], [1, 1, 0]];
    let days = vec_vec_i32![[7, 0, 0], [0, 7, 0], [0, 0, 7]];
    let res = 21;
    assert_eq!(Solution::max_vacation_days(flights, days), res);
}
