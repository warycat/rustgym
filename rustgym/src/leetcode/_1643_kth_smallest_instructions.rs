struct Solution;

impl Solution {
    fn kth_smallest_path(destination: Vec<i32>, k: i32) -> String {
        let n = (destination[0] + 1) as usize;
        let m = (destination[1] + 1) as usize;
        let mut res = "".to_string();
        let mut dp = vec![vec![0; m]; n];
        for i in (0..n).rev() {
            for j in (0..m).rev() {
                if i == n - 1 && j == m - 1 {
                    dp[i][j] = 1;
                    continue;
                }
                if i == n - 1 {
                    dp[i][j] = dp[i][j + 1];
                    continue;
                }
                if j == m - 1 {
                    dp[i][j] = dp[i + 1][j];
                    continue;
                }
                dp[i][j] = dp[i][j + 1] + dp[i + 1][j];
            }
        }
        Self::build(0, 0, k, &mut res, &dp, n, m);
        res
    }

    fn build(i: usize, j: usize, k: i32, path: &mut String, dp: &[Vec<i32>], n: usize, m: usize) {
        if i == n - 1 && j == m - 1 {
            return;
        }
        if i == n - 1 {
            path.push('H');
            Self::build(i, j + 1, k, path, dp, n, m);
            return;
        }
        if j == m - 1 {
            path.push('V');
            Self::build(i + 1, j, k, path, dp, n, m);
            return;
        }
        if k <= dp[i][j + 1] {
            path.push('H');
            Self::build(i, j + 1, k, path, dp, n, m);
        } else {
            path.push('V');
            Self::build(i + 1, j, k - dp[i][j + 1], path, dp, n, m);
        }
    }
}

#[test]
fn test() {
    let destination = vec![2, 3];
    let k = 1;
    let res = "HHHVV".to_string();
    assert_eq!(Solution::kth_smallest_path(destination, k), res);
    let destination = vec![2, 3];
    let k = 2;
    let res = "HHVHV".to_string();
    assert_eq!(Solution::kth_smallest_path(destination, k), res);
    let destination = vec![2, 3];
    let k = 3;
    let res = "HHVVH".to_string();
    assert_eq!(Solution::kth_smallest_path(destination, k), res);
}
