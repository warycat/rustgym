struct Solution;

use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    fn minimum_effort_path(heights: Vec<Vec<i32>>) -> i32 {
        let n = heights.len();
        let m = heights[0].len();
        let mut visited: Vec<Vec<bool>> = vec![vec![false; m]; n];
        let mut queue: BinaryHeap<(Reverse<i32>, usize, usize)> = BinaryHeap::new();
        let mut res = 0;
        queue.push((Reverse(0), 0, 0));
        while let Some((Reverse(effort), i, j)) = queue.pop() {
            res = res.max(effort);
            if i == n - 1 && j == m - 1 {
                break;
            }
            visited[i][j] = true;
            if i > 0 && !visited[i - 1][j] {
                let diff = heights[i][j] - heights[i - 1][j];
                queue.push((Reverse(diff.abs()), i - 1, j));
            }
            if j > 0 && !visited[i][j - 1] {
                let diff = heights[i][j] - heights[i][j - 1];
                queue.push((Reverse(diff.abs()), i, j - 1));
            }
            if i + 1 < n && !visited[i + 1][j] {
                let diff = heights[i][j] - heights[i + 1][j];
                queue.push((Reverse(diff.abs()), i + 1, j));
            }
            if j + 1 < m && !visited[i][j + 1] {
                let diff = heights[i][j] - heights[i][j + 1];
                queue.push((Reverse(diff.abs()), i, j + 1));
            }
        }
        res
    }
}

#[test]
fn test() {
    let heights = vec_vec_i32![[1, 2, 2], [3, 8, 2], [5, 3, 5]];
    let res = 2;
    assert_eq!(Solution::minimum_effort_path(heights), res);
    let heights = vec_vec_i32![[1, 2, 3], [3, 8, 4], [5, 3, 5]];
    let res = 1;
    assert_eq!(Solution::minimum_effort_path(heights), res);
    let heights = vec_vec_i32![
        [1, 2, 1, 1, 1],
        [1, 2, 1, 2, 1],
        [1, 2, 1, 2, 1],
        [1, 2, 1, 2, 1],
        [1, 1, 1, 2, 1]
    ];
    let res = 0;
    assert_eq!(Solution::minimum_effort_path(heights), res);
}
