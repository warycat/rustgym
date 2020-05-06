struct Solution;

impl Solution {
    fn unique_paths_iii(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut res = 0;
        let n = grid.len();
        let m = grid[0].len();
        let mut count = 0;
        for i in 0..n {
            for j in 0..m {
                if grid[i][j] == 0 {
                    count += 1;
                }
            }
        }
        for i in 0..n {
            for j in 0..m {
                if grid[i][j] == 1 {
                    Self::dfs(i, j, count, &mut res, &mut grid, n, m);
                }
            }
        }
        res as i32
    }

    fn dfs(
        i: usize,
        j: usize,
        left: usize,
        all: &mut usize,
        grid: &mut Vec<Vec<i32>>,
        n: usize,
        m: usize,
    ) {
        match grid[i][j] {
            2 => {
                if left == 0 {
                    *all += 1;
                }
            }
            1 => {
                grid[i][j] = -1;
                for (r, c) in Self::adj(i, j, n, m) {
                    Self::dfs(r, c, left, all, grid, n, m);
                }
                grid[i][j] = 1;
            }

            0 => {
                grid[i][j] = -1;
                for (r, c) in Self::adj(i, j, n, m) {
                    Self::dfs(r, c, left - 1, all, grid, n, m);
                }
                grid[i][j] = 0;
            }

            _ => {}
        }
    }

    fn adj(i: usize, j: usize, n: usize, m: usize) -> Vec<(usize, usize)> {
        let mut res = vec![];
        if i > 0 {
            res.push((i - 1, j));
        }
        if j > 0 {
            res.push((i, j - 1));
        }
        if i + 1 < n {
            res.push((i + 1, j));
        }
        if j + 1 < m {
            res.push((i, j + 1));
        }
        res
    }
}

#[test]
fn test() {
    let grid = vec_vec_i32![[1, 0, 0, 0], [0, 0, 0, 0], [0, 0, 2, -1]];
    let res = 2;
    assert_eq!(Solution::unique_paths_iii(grid), res);
    let grid = vec_vec_i32![[1, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 2]];
    let res = 4;
    assert_eq!(Solution::unique_paths_iii(grid), res);
    let grid = vec_vec_i32![[0, 1], [2, 0]];
    let res = 0;
    assert_eq!(Solution::unique_paths_iii(grid), res);
}
