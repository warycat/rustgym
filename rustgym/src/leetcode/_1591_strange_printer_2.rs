struct Solution;

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

impl Solution {
    fn is_printable(target_grid: Vec<Vec<i32>>) -> bool {
        let n = target_grid.len();
        let m = target_grid[0].len();
        let mut color: HashMap<i32, usize> = HashMap::new();
        for i in 0..n {
            for j in 0..m {
                let size = color.len();
                color.entry(target_grid[i][j]).or_insert(size);
            }
        }
        let c = color.len();
        let mut left: Vec<usize> = vec![m - 1; c];
        let mut right: Vec<usize> = vec![0; c];
        let mut top: Vec<usize> = vec![n - 1; c];
        let mut bottom: Vec<usize> = vec![0; c];
        for i in 0..n {
            for j in 0..m {
                let u = color[&target_grid[i][j]];
                left[u] = left[u].min(j);
                right[u] = right[u].max(j);
                top[u] = top[u].min(i);
                bottom[u] = bottom[u].max(i);
            }
        }
        let mut adj: Vec<HashSet<usize>> = vec![HashSet::new(); c];
        for i in 0..n {
            for j in 0..m {
                let u = color[&target_grid[i][j]];
                for v in 0..c {
                    if v == u {
                        continue;
                    }
                    if left[v] <= j && j <= right[v] && top[v] <= i && i <= bottom[v] {
                        adj[v].insert(u);
                    }
                }
            }
        }
        let mut indegree = vec![0; c];
        for u in 0..c {
            for &v in &adj[u] {
                indegree[v] += 1;
            }
        }
        let mut queue: VecDeque<usize> = VecDeque::new();
        let mut visited = 0;
        for u in 0..c {
            if indegree[u] == 0 {
                visited += 1;
                queue.push_back(u);
            }
        }
        while let Some(u) = queue.pop_front() {
            for &v in &adj[u] {
                indegree[v] -= 1;
                if indegree[v] == 0 {
                    visited += 1;
                    queue.push_back(v);
                }
            }
        }
        visited == c
    }
}

#[test]
fn test() {
    let target_grid = vec_vec_i32![[1, 1, 1, 1], [1, 2, 2, 1], [1, 2, 2, 1], [1, 1, 1, 1]];
    let res = true;
    assert_eq!(Solution::is_printable(target_grid), res);
    let target_grid = vec_vec_i32![[1, 1, 1, 1], [1, 1, 3, 3], [1, 1, 3, 4], [5, 5, 1, 4]];
    let res = true;
    assert_eq!(Solution::is_printable(target_grid), res);
    let target_grid = vec_vec_i32![[1, 2, 1], [2, 1, 2], [1, 2, 1]];
    let res = false;
    assert_eq!(Solution::is_printable(target_grid), res);
    let target_grid = vec_vec_i32![[1, 1, 1], [3, 1, 3]];
    let res = false;
    assert_eq!(Solution::is_printable(target_grid), res);
}
