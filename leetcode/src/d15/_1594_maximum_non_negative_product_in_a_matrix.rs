struct Solution;

const MOD: i64 = 1_000_000_007;

impl Solution {
    fn max_product_path(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();
        let mut max = vec![vec![0; m]; n];
        let mut min = vec![vec![0; m]; n];
        max[0][0] = grid[0][0] as i64;
        min[0][0] = grid[0][0] as i64;
        for i in 1..n {
            max[i][0] = max[i - 1][0] * grid[i][0] as i64;
            min[i][0] = min[i - 1][0] * grid[i][0] as i64;
        }
        for j in 1..m {
            max[0][j] = max[0][j - 1] * grid[0][j] as i64;
            min[0][j] = min[0][j - 1] * grid[0][j] as i64;
        }
        for i in 1..n {
            for j in 1..m {
                if grid[i][j] < 0 {
                    max[i][j] = grid[i][j] as i64 * min[i][j - 1].min(min[i - 1][j]);
                    min[i][j] = grid[i][j] as i64 * max[i][j - 1].max(max[i - 1][j]);
                } else {
                    max[i][j] = grid[i][j] as i64 * max[i][j - 1].max(max[i - 1][j]);
                    min[i][j] = grid[i][j] as i64 * min[i][j - 1].min(min[i - 1][j]);
                }
            }
        }

        if max[n - 1][m - 1] < 0 {
            -1
        } else {
            (max[n - 1][m - 1] % MOD) as i32
        }
    }
}

#[test]
fn test() {
    let grid = vec_vec_i32![[-1, -2, -3], [-2, -3, -3], [-3, -3, -2]];
    let res = -1;
    assert_eq!(Solution::max_product_path(grid), res);
    let grid = vec_vec_i32![[1, -2, 1], [1, -2, 1], [3, -4, 1]];
    let res = 8;
    assert_eq!(Solution::max_product_path(grid), res);
    let grid = vec_vec_i32![[1, 3], [0, -4]];
    let res = 0;
    assert_eq!(Solution::max_product_path(grid), res);
    let grid = vec_vec_i32![[1, 4, 4, 0], [-2, 0, 0, 1], [1, -1, 1, 1]];
    let res = 2;
    assert_eq!(Solution::max_product_path(grid), res);
}
