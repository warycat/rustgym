struct Solution;

use std::collections::HashMap;

impl Solution {
    fn largest_island(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut group = vec![vec![0; n]; n];
        let mut gid = 0;
        let mut group_size = vec![0];
        for i in 0..n {
            for j in 0..n {
                if grid[i][j] == 1 && group[i][j] == 0 {
                    gid += 1;
                    group_size.push(0);
                    Self::dfs(i, j, gid, &mut group, &mut group_size, &grid, n);
                }
            }
        }
        let mut res = *group_size.iter().max().unwrap_or(&0);
        for i in 0..n {
            for j in 0..n {
                if grid[i][j] == 0 {
                    let mut groups: HashMap<usize, usize> = HashMap::new();
                    if i > 0 {
                        let gid = group[i - 1][j];
                        let size = group_size[gid];
                        groups.entry(gid).or_insert(size);
                    }
                    if j > 0 {
                        let gid = group[i][j - 1];
                        let size = group_size[gid];
                        groups.entry(gid).or_insert(size);
                    }
                    if i + 1 < n {
                        let gid = group[i + 1][j];
                        let size = group_size[gid];
                        groups.entry(gid).or_insert(size);
                    }
                    if j + 1 < n {
                        let gid = group[i][j + 1];
                        let size = group_size[gid];
                        groups.entry(gid).or_insert(size);
                    }
                    res = res.max(groups.values().sum::<usize>() + 1);
                }
            }
        }
        res as i32
    }

    fn dfs(
        i: usize,
        j: usize,
        gid: usize,
        group: &mut Vec<Vec<usize>>,
        group_size: &mut Vec<usize>,
        grid: &[Vec<i32>],
        n: usize,
    ) {
        group[i][j] = gid;
        group_size[gid] += 1;
        if i > 0 && grid[i - 1][j] == 1 && group[i - 1][j] == 0 {
            Self::dfs(i - 1, j, gid, group, group_size, grid, n);
        }
        if j > 0 && grid[i][j - 1] == 1 && group[i][j - 1] == 0 {
            Self::dfs(i, j - 1, gid, group, group_size, grid, n);
        }
        if i + 1 < n && grid[i + 1][j] == 1 && group[i + 1][j] == 0 {
            Self::dfs(i + 1, j, gid, group, group_size, grid, n);
        }
        if j + 1 < n && grid[i][j + 1] == 1 && group[i][j + 1] == 0 {
            Self::dfs(i, j + 1, gid, group, group_size, grid, n);
        }
    }
}

#[test]
fn test() {
    let grid = vec_vec_i32![[1, 0], [0, 1]];
    let res = 3;
    assert_eq!(Solution::largest_island(grid), res);
    let grid = vec_vec_i32![[1, 1], [1, 0]];
    let res = 4;
    assert_eq!(Solution::largest_island(grid), res);
    let grid = vec_vec_i32![[1, 1], [1, 1]];
    let res = 4;
    assert_eq!(Solution::largest_island(grid), res);
    let grid = vec_vec_i32![[1, 0], [1, 0]];
    let res = 3;
    assert_eq!(Solution::largest_island(grid), res);
}
