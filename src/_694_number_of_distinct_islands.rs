struct Solution;
use std::collections::HashSet;

#[derive(Hash, PartialEq, Eq, Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
    Back,
}

type Island = Vec<Direction>;

impl Solution {
    fn num_distinct_islands(mut grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();
        let mut hs: HashSet<Island> = HashSet::new();
        for i in 0..n {
            for j in 0..m {
                if grid[i][j] == 1 {
                    let mut island: Island = vec![];
                    Self::dfs(i, j, &mut grid, &mut island, n, m);
                    hs.insert(island);
                }
            }
        }
        hs.len() as i32
    }

    fn dfs(i: usize, j: usize, grid: &mut Vec<Vec<i32>>, island: &mut Island, n: usize, m: usize) {
        use Direction::*;
        grid[i][j] = 0;
        if i > 0 && grid[i - 1][j] == 1 {
            island.push(Up);
            Self::dfs(i - 1, j, grid, island, n, m);
        }
        if j > 0 && grid[i][j - 1] == 1 {
            island.push(Left);
            Self::dfs(i, j - 1, grid, island, n, m);
        }
        if i + 1 < n && grid[i + 1][j] == 1 {
            island.push(Down);
            Self::dfs(i + 1, j, grid, island, n, m);
        }
        if j + 1 < m && grid[i][j + 1] == 1 {
            island.push(Right);
            Self::dfs(i, j + 1, grid, island, n, m);
        }
        island.push(Back);
    }
}

#[test]
fn test() {
    let grid = vec_vec_i32![
        [1, 1, 0, 0, 0],
        [1, 1, 0, 0, 0],
        [0, 0, 0, 1, 1],
        [0, 0, 0, 1, 1]
    ];
    let res = 1;
    assert_eq!(Solution::num_distinct_islands(grid), res);
    let grid = vec_vec_i32![
        [1, 1, 0, 1, 1],
        [1, 0, 0, 0, 0],
        [0, 0, 0, 0, 1],
        [1, 1, 0, 1, 1]
    ];
    let res = 3;
    assert_eq!(Solution::num_distinct_islands(grid), res);
    let grid = vec_vec_i32![[1, 1, 0], [0, 1, 1], [0, 0, 0], [1, 1, 1], [0, 1, 0]];
    let res = 2;
    assert_eq!(Solution::num_distinct_islands(grid), res);
}
