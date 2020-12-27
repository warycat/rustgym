struct Solution;

impl Solution {
    fn find_ball(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let n = grid.len();
        let m = grid[0].len();
        let mut res = vec![];
        for j in 0..m {
            res.push(Self::dp(0, j, &grid, n, m));
        }
        res
    }

    fn dp(i: usize, j: usize, grid: &[Vec<i32>], n: usize, m: usize) -> i32 {
        if i == n {
            j as i32
        } else {
            if grid[i][j] == 1 && j + 1 < m && grid[i][j + 1] == 1 {
                return Self::dp(i + 1, j + 1, grid, n, m);
            }
            if grid[i][j] == -1 && j >= 1 && grid[i][j - 1] == -1 {
                return Self::dp(i + 1, j - 1, grid, n, m);
            }
            -1
        }
    }
}

#[test]
fn test() {
    let grid = vec_vec_i32![
        [1, 1, 1, -1, -1],
        [1, 1, 1, -1, -1],
        [-1, -1, -1, 1, 1],
        [1, 1, 1, 1, -1],
        [-1, -1, -1, -1, -1]
    ];
    let res = vec![1, -1, -1, -1, -1];
    assert_eq!(Solution::find_ball(grid), res);
}
