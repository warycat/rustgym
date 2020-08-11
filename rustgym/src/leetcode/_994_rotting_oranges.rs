struct Solution;

use std::collections::VecDeque;

struct Orange {
    r: usize,
    c: usize,
    t: i32,
}

impl Solution {
    fn oranges_rotting(mut grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();
        let mut queue: VecDeque<Orange> = VecDeque::new();
        for i in 0..n {
            for j in 0..m {
                if grid[i][j] == 2 {
                    queue.push_back(Orange { r: i, c: j, t: 0 });
                }
            }
        }
        let mut res = 0;
        while let Some(o) = queue.pop_front() {
            let r = o.r;
            let c = o.c;
            let t = o.t;
            if r > 0 && grid[r - 1][c] == 1 {
                grid[r - 1][c] = 2;
                res = i32::max(res, t + 1);
                queue.push_back(Orange {
                    r: r - 1,
                    c,
                    t: t + 1,
                });
            }
            if r < n - 1 && grid[r + 1][c] == 1 {
                grid[r + 1][c] = 2;
                res = i32::max(res, t + 1);
                queue.push_back(Orange {
                    r: r + 1,
                    c,
                    t: t + 1,
                });
            }
            if c > 0 && grid[r][c - 1] == 1 {
                grid[r][c - 1] = 2;
                res = i32::max(res, t + 1);
                queue.push_back(Orange {
                    r,
                    c: c - 1,
                    t: t + 1,
                });
            }
            if c < m - 1 && grid[r][c + 1] == 1 {
                grid[r][c + 1] = 2;
                res = i32::max(res, t + 1);
                queue.push_back(Orange {
                    r,
                    c: c + 1,
                    t: t + 1,
                });
            }
        }
        for i in 0..n {
            for j in 0..m {
                if grid[i][j] == 1 {
                    return -1;
                }
            }
        }
        res
    }
}

#[test]
fn test() {
    let grid: Vec<Vec<i32>> = vec_vec_i32![[2, 1, 1], [1, 1, 0], [0, 1, 1]];
    assert_eq!(Solution::oranges_rotting(grid), 4);
    let grid: Vec<Vec<i32>> = vec_vec_i32![[2, 1, 1], [0, 1, 1], [1, 0, 1]];
    assert_eq!(Solution::oranges_rotting(grid), -1);
    let grid: Vec<Vec<i32>> = vec_vec_i32![[0, 2]];
    assert_eq!(Solution::oranges_rotting(grid), 0);
}
