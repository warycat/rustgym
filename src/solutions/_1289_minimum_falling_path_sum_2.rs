struct Solution;

impl Solution {
    fn min_falling_path_sum(arr: Vec<Vec<i32>>) -> i32 {
        let n = arr.len();
        let m = arr[0].len();
        let mut dp = vec![vec![0; m]; n + 1];
        for i in 0..n {
            for j in 0..m {
                let mut min = std::i32::MAX;
                for k in 0..m {
                    if k != j {
                        min = min.min(dp[i][k]);
                    }
                }
                dp[i + 1][j] = arr[i][j] + min;
            }
        }
        *dp[n].iter().min().unwrap()
    }
}

#[test]
fn test() {
    let arr = vec_vec_i32![[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    let res = 13;
    assert_eq!(Solution::min_falling_path_sum(arr), res);
}
