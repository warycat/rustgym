struct Solution;

impl Solution {
    fn min_cost_ii(costs: Vec<Vec<i32>>) -> i32 {
        let n = costs.len();
        if n == 0 {
            return 0;
        }
        let m = costs[0].len();
        let mut dp = vec![vec![0; m]; n];
        for i in 0..m {
            dp[0][i] = costs[0][i];
        }
        for i in 1..n {
            for j in 0..m {
                let mut min = std::i32::MAX;
                for k in 0..m {
                    if k != j {
                        min = min.min(dp[i - 1][k]);
                    }
                }
                dp[i][j] = costs[i][j] + min;
            }
        }
        dp[n - 1].iter().copied().min().unwrap()
    }
}

#[test]
fn test() {
    let costs = vec_vec_i32![[1, 5, 3], [2, 9, 4]];
    let res = 5;
    assert_eq!(Solution::min_cost_ii(costs), res);
}
