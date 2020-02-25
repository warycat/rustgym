struct Solution;

impl Solution {
    fn min_falling_path_sum(a: Vec<Vec<i32>>) -> i32 {
        let n = a.len();
        let m = a[0].len();
        let mut dp = vec![vec![0; m]; n];
        for i in 0..n {
            for j in 0..m {
                let mut min = std::i32::MAX;
                if i > 0 {
                    let l = 0.max(j as i32 - 1) as usize;
                    let r = (n - 1).min(j + 1);
                    for k in l..=r {
                        min = min.min(dp[i - 1][k]);
                    }
                } else {
                    min = 0;
                }
                dp[i][j] = a[i][j] + min;
            }
        }
        *dp[n - 1].iter().min().unwrap()
    }
}

#[test]
fn test() {
    let a = vec_vec_i32![[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    let res = 12;
    assert_eq!(Solution::min_falling_path_sum(a), res);
}
