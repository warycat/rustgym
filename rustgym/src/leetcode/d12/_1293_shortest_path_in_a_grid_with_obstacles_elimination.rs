struct Solution;

use std::collections::HashSet;
use std::collections::VecDeque;

impl Solution {
    fn shortest_path(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let n = grid.len();
        let m = grid[0].len();
        let mut queue: VecDeque<(usize, usize, i32, i32)> = VecDeque::new();
        let mut visited: HashSet<(usize, usize, i32)> = HashSet::new();
        queue.push_back((0, 0, k, 0));
        visited.insert((0, 0, k));
        while let Some((i, j, left, step)) = queue.pop_front() {
            if i == n - 1 && j == m - 1 {
                return step;
            }
            let nstep = step + 1;
            for &(di, dj) in &[(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let ni = i as i32 + di;
                let nj = j as i32 + dj;
                if 0 <= ni && ni < n as i32 && 0 <= nj && nj < m as i32 {
                    let ni = ni as usize;
                    let nj = nj as usize;
                    let nleft = left - grid[ni][nj];
                    if nleft >= 0 && visited.insert((ni, nj, nleft)) {
                        queue.push_back((ni, nj, nleft, nstep));
                    }
                }
            }
        }
        -1
    }
}

#[test]
fn test() {
    let grid = vec_vec_i32![[0, 0, 0], [1, 1, 0], [0, 0, 0], [0, 1, 1], [0, 0, 0]];
    let k = 1;
    let res = 6;
    assert_eq!(Solution::shortest_path(grid, k), res);
    let grid = vec_vec_i32![[0, 1, 1], [1, 1, 1], [1, 0, 0]];
    let k = 1;
    let res = -1;
    assert_eq!(Solution::shortest_path(grid, k), res);
}
