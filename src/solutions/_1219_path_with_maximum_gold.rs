struct Solution;

impl Solution {
    fn get_maximum_gold(mut grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();
        let mut sum = 0;
        let mut res = 0;
        for i in 0..n {
            for j in 0..m {
                Self::dfs(i, j, &mut sum, &mut res, &mut grid, n, m);
            }
        }
        res
    }

    fn dfs(
        i: usize,
        j: usize,
        sum: &mut i32,
        max: &mut i32,
        grid: &mut Vec<Vec<i32>>,
        n: usize,
        m: usize,
    ) {
        if grid[i][j] == 0 {
            return;
        }
        let val = grid[i][j];
        *sum += val;
        *max = (*max).max(*sum);
        grid[i][j] = 0;
        if i > 0 {
            Self::dfs(i - 1, j, sum, max, grid, n, m);
        }
        if j > 0 {
            Self::dfs(i, j - 1, sum, max, grid, n, m);
        }
        if i + 1 < n {
            Self::dfs(i + 1, j, sum, max, grid, n, m);
        }
        if j + 1 < m {
            Self::dfs(i, j + 1, sum, max, grid, n, m);
        }
        grid[i][j] = val;
        *sum -= grid[i][j];
    }
}

#[test]
fn test() {
    let grid = vec_vec_i32![[0, 6, 0], [5, 8, 7], [0, 9, 0]];
    let res = 24;
    assert_eq!(Solution::get_maximum_gold(grid), res);
    let grid = vec_vec_i32![[1, 0, 7], [2, 0, 6], [3, 4, 5], [0, 3, 0], [9, 0, 20]];
    let res = 28;
    assert_eq!(Solution::get_maximum_gold(grid), res);
}
