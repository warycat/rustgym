struct Solution;

use std::collections::VecDeque;

impl Solution {
    fn min_cost(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();
        let mut dist = vec![vec![std::i32::MAX; m]; n];
        let mut queue: VecDeque<(usize, usize, i32)> = VecDeque::new();
        dist[0][0] = 0;
        queue.push_back((0, 0, 0));
        while let Some((i, j, d)) = queue.pop_front() {
            let right = Self::cost(i, j, d, 1, &grid);
            let left = Self::cost(i, j, d, 2, &grid);
            let down = Self::cost(i, j, d, 3, &grid);
            let up = Self::cost(i, j, d, 4, &grid);
            if i > 0 && up < dist[i - 1][j] {
                dist[i - 1][j] = up;
                queue.push_back((i - 1, j, up));
            }
            if j > 0 && left < dist[i][j - 1] {
                dist[i][j - 1] = left;
                queue.push_back((i, j - 1, left));
            }
            if i + 1 < n && down < dist[i + 1][j] {
                dist[i + 1][j] = down;
                queue.push_back((i + 1, j, down));
            }
            if j + 1 < m && right < dist[i][j + 1] {
                dist[i][j + 1] = right;
                queue.push_back((i, j + 1, right));
            }
        }
        dist[n - 1][m - 1]
    }

    fn cost(i: usize, j: usize, cost: i32, dir: i32, grid: &[Vec<i32>]) -> i32 {
        if dir == grid[i][j] {
            cost
        } else {
            cost + 1
        }
    }
}

#[test]
fn test() {
    let grid = vec_vec_i32![[1, 1, 1, 1], [2, 2, 2, 2], [1, 1, 1, 1], [2, 2, 2, 2]];
    let res = 3;
    assert_eq!(Solution::min_cost(grid), res);
    let grid = vec_vec_i32![[1, 1, 3], [3, 2, 2], [1, 1, 4]];
    let res = 0;
    assert_eq!(Solution::min_cost(grid), res);
    let grid = vec_vec_i32![[2, 2, 2], [2, 2, 2]];
    let res = 3;
    assert_eq!(Solution::min_cost(grid), res);
    let grid = vec_vec_i32![[4]];
    let res = 0;
    assert_eq!(Solution::min_cost(grid), res);
}
