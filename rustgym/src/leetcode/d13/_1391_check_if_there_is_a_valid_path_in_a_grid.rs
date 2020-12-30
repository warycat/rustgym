struct Solution;

use std::collections::VecDeque;

trait Connect {
    fn top(self) -> bool;
    fn left(self) -> bool;
    fn bottom(self) -> bool;
    fn right(self) -> bool;
}

impl Connect for i32 {
    fn top(self) -> bool {
        matches!(self, 2 | 5 | 6)
    }
    fn left(self) -> bool {
        matches!(self, 1 | 3 | 5)
    }
    fn bottom(self) -> bool {
        matches!(self, 2 | 3 | 4)
    }
    fn right(self) -> bool {
        matches!(self, 1 | 4 | 6)
    }
}

impl Solution {
    fn has_valid_path(grid: Vec<Vec<i32>>) -> bool {
        let n = grid.len();
        let m = grid[0].len();
        let mut visited = vec![vec![false; m]; n];
        let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
        queue.push_back((0, 0));
        while let Some((i, j)) = queue.pop_front() {
            visited[i][j] = true;
            if visited[n - 1][m - 1] {
                return true;
            }
            if i > 0 && !visited[i - 1][j] && grid[i][j].top() && grid[i - 1][j].bottom() {
                queue.push_back((i - 1, j));
            }
            if j > 0 && !visited[i][j - 1] && grid[i][j].left() && grid[i][j - 1].right() {
                queue.push_back((i, j - 1));
            }
            if i + 1 < n && !visited[i + 1][j] && grid[i][j].bottom() && grid[i + 1][j].top() {
                queue.push_back((i + 1, j));
            }
            if j + 1 < m && !visited[i][j + 1] && grid[i][j].right() && grid[i][j + 1].left() {
                queue.push_back((i, j + 1));
            }
        }
        false
    }
}

#[test]
fn test() {
    let grid = vec_vec_i32![[2, 4, 3], [6, 5, 2]];
    let res = true;
    assert_eq!(Solution::has_valid_path(grid), res);
    let grid = vec_vec_i32![[1, 2, 1], [1, 2, 1]];
    let res = false;
    assert_eq!(Solution::has_valid_path(grid), res);
    let grid = vec_vec_i32![[1, 1, 2]];
    let res = false;
    assert_eq!(Solution::has_valid_path(grid), res);
    let grid = vec_vec_i32![[1, 1, 1, 1, 1, 1, 3]];
    let res = true;
    assert_eq!(Solution::has_valid_path(grid), res);
    let grid = vec_vec_i32![[2], [2], [2], [2], [2], [2], [6]];
    let res = true;
    assert_eq!(Solution::has_valid_path(grid), res);
}
