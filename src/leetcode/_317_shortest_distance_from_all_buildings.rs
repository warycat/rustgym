struct Solution;

use std::collections::VecDeque;

impl Solution {
    fn shortest_distance(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();
        let mut dist = vec![vec![0; m]; n];
        let mut count = vec![vec![0; m]; n];
        let mut building = 0;
        for i in 0..n {
            for j in 0..m {
                if grid[i][j] == 1 {
                    building += 1;
                }
            }
        }
        for i in 0..n {
            for j in 0..m {
                if grid[i][j] == 1 {
                    Self::bfs(i, j, &mut dist, &mut count, &grid, m, n)
                }
            }
        }
        let mut res = std::usize::MAX;
        for i in 0..n {
            for j in 0..m {
                if grid[i][j] == 0 && count[i][j] == building {
                    res = res.min(dist[i][j]);
                }
            }
        }
        if res == std::usize::MAX {
            -1
        } else {
            res as i32
        }
    }

    fn bfs(
        i: usize,
        j: usize,
        dist: &mut Vec<Vec<usize>>,
        count: &mut Vec<Vec<usize>>,
        grid: &[Vec<i32>],
        m: usize,
        n: usize,
    ) {
        let mut visited = vec![vec![false; m]; n];
        let mut queue: VecDeque<(usize, usize, usize)> = VecDeque::new();
        queue.push_back((i, j, 0));
        while let Some((i, j, d)) = queue.pop_front() {
            if i > 0 && grid[i - 1][j] == 0 && !visited[i - 1][j] {
                visited[i - 1][j] = true;
                dist[i - 1][j] += d + 1;
                count[i - 1][j] += 1;
                queue.push_back((i - 1, j, d + 1));
            }
            if j > 0 && grid[i][j - 1] == 0 && !visited[i][j - 1] {
                visited[i][j - 1] = true;
                dist[i][j - 1] += d + 1;
                count[i][j - 1] += 1;
                queue.push_back((i, j - 1, d + 1));
            }
            if i + 1 < n && grid[i + 1][j] == 0 && !visited[i + 1][j] {
                visited[i + 1][j] = true;
                dist[i + 1][j] += d + 1;
                count[i + 1][j] += 1;
                queue.push_back((i + 1, j, d + 1));
            }
            if j + 1 < m && grid[i][j + 1] == 0 && !visited[i][j + 1] {
                visited[i][j + 1] = true;
                dist[i][j + 1] += d + 1;
                count[i][j + 1] += 1;
                queue.push_back((i, j + 1, d + 1));
            }
        }
    }
}

#[test]
fn test() {
    let grid = vec_vec_i32![[1, 0, 2, 0, 1], [0, 0, 0, 0, 0], [0, 0, 1, 0, 0]];
    let res = 7;
    assert_eq!(Solution::shortest_distance(grid), res);
    let grid = vec_vec_i32![
        [1, 1, 1, 1, 1, 0],
        [0, 0, 0, 0, 0, 1],
        [0, 1, 1, 0, 0, 1],
        [1, 0, 0, 1, 0, 1],
        [1, 0, 1, 0, 0, 1],
        [1, 0, 0, 0, 0, 1],
        [0, 1, 1, 1, 1, 0]
    ];
    let res = 88;
    assert_eq!(Solution::shortest_distance(grid), res);
}
