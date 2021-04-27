struct Solution;
use std::collections::VecDeque;

impl Solution {
    fn maximum_invitations(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();
        let mut graph: Vec<Vec<i32>> = vec![vec![0; n + m + 2]; n + m + 2];
        for i in 0..n {
            let u = 0;
            let v = 1 + i;
            graph[u][v] = 1;
        }
        for j in 0..m {
            let u = 1 + n + j;
            let v = 1 + n + m;
            graph[u][v] = 1;
        }
        for i in 0..n {
            for j in 0..m {
                let u = 1 + i;
                let v = 1 + n + j;
                if grid[i][j] == 1 {
                    graph[u][v] = 1;
                }
            }
        }
        let mut res = 0;
        while let Some(path) = bfs(&graph) {
            update(&mut graph, path);
            res += 1;
        }
        res
    }
}

fn bfs(graph: &[Vec<i32>]) -> Option<Vec<usize>> {
    let n = graph.len();
    let mut visited = vec![false; n];
    let mut queue: VecDeque<usize> = VecDeque::new();
    queue.push_back(0);
    visited[0] = true;
    let mut res = vec![0; n];
    while let Some(u) = queue.pop_front() {
        for v in 0..n {
            if !visited[v] && graph[u][v] == 1 {
                visited[v] = true;
                queue.push_back(v);
                res[v] = u;
            }
        }
    }
    if visited[n - 1] {
        Some(res)
    } else {
        None
    }
}

fn update(graph: &mut Vec<Vec<i32>>, path: Vec<usize>) {
    let n = graph.len();
    let mut v = n - 1;
    loop {
        let u = path[v];
        graph[u][v] -= 1;
        graph[v][u] += 1;
        v = u;
        if u == 0 {
            break;
        }
    }
}

#[test]
fn test() {
    let grid = vec_vec_i32![[1, 1, 1], [1, 0, 1], [0, 0, 1]];
    let res = 3;
    assert_eq!(Solution::maximum_invitations(grid), res);
    let grid = vec_vec_i32![[1, 0, 1, 0], [1, 0, 0, 0], [0, 0, 1, 0], [1, 1, 1, 0]];
    let res = 3;
    assert_eq!(Solution::maximum_invitations(grid), res);
}
