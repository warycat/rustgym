struct Solution;

impl Solution {
    fn color_border(mut grid: Vec<Vec<i32>>, r0: i32, c0: i32, color: i32) -> Vec<Vec<i32>> {
        let n = grid.len();
        let m = grid[0].len();
        let r0 = r0 as usize;
        let c0 = c0 as usize;
        let c_color = grid[r0][c0];
        let b_color = color;
        let mut visited: Vec<Vec<bool>> = vec![vec![false; m]; n];
        Self::dfs(r0, c0, &mut visited, &mut grid, b_color, c_color, n, m);
        grid
    }
    fn dfs(
        i: usize,
        j: usize,
        visited: &mut [Vec<bool>],
        grid: &mut [Vec<i32>],
        b_color: i32,
        c_color: i32,
        n: usize,
        m: usize,
    ) {
        visited[i][j] = true;
        let top = if i == 0 {
            true
        } else {
            if grid[i - 1][j] != c_color {
                !visited[i - 1][j]
            } else {
                if !visited[i - 1][j] {
                    Self::dfs(i - 1, j, visited, grid, b_color, c_color, n, m);
                }
                false
            }
        };
        let left = if j == 0 {
            true
        } else {
            if grid[i][j - 1] != c_color {
                !visited[i][j - 1]
            } else {
                if !visited[i][j - 1] {
                    Self::dfs(i, j - 1, visited, grid, b_color, c_color, n, m);
                }
                false
            }
        };
        let down = if i + 1 == n {
            true
        } else {
            if grid[i + 1][j] != c_color {
                !visited[i + 1][j]
            } else {
                if !visited[i + 1][j] {
                    Self::dfs(i + 1, j, visited, grid, b_color, c_color, n, m);
                }
                false
            }
        };
        let right = if j + 1 == m {
            true
        } else {
            if grid[i][j + 1] != c_color {
                !visited[i][j + 1]
            } else {
                if !visited[i][j + 1] {
                    Self::dfs(i, j + 1, visited, grid, b_color, c_color, n, m);
                }
                false
            }
        };
        if top || left || down || right {
            grid[i][j] = b_color;
        }
    }
}

#[test]
fn test() {
    let grid = vec_vec_i32![[1, 1], [1, 2]];
    let r0 = 0;
    let c0 = 0;
    let color = 3;
    let res = vec_vec_i32![[3, 3], [3, 2]];
    assert_eq!(Solution::color_border(grid, r0, c0, color), res);
    let grid = vec_vec_i32![[1, 2, 2], [2, 3, 2]];
    let r0 = 0;
    let c0 = 1;
    let color = 3;
    let res = vec_vec_i32![[1, 3, 3], [2, 3, 3]];
    assert_eq!(Solution::color_border(grid, r0, c0, color), res);
    let grid = vec_vec_i32![[1, 1, 1], [1, 1, 1], [1, 1, 1]];
    let r0 = 1;
    let c0 = 1;
    let color = 2;
    let res = vec_vec_i32![[2, 2, 2], [2, 1, 2], [2, 2, 2]];
    assert_eq!(Solution::color_border(grid, r0, c0, color), res);
}
