struct Solution;
use std::collections::VecDeque;

impl Solution {
    fn shortest_alternating_paths(
        n: i32,
        red_edges: Vec<Vec<i32>>,
        blue_edges: Vec<Vec<i32>>,
    ) -> Vec<i32> {
        let n = n as usize;
        let mut graph: Vec<Vec<Vec<usize>>> = vec![vec![vec![]; n]; 2];
        let mut visited: Vec<Vec<bool>> = vec![vec![false; n]; 2];
        let mut res: Vec<i32> = vec![-1; n];
        for e in red_edges {
            let u = e[0] as usize;
            let v = e[1] as usize;
            graph[0][u].push(v);
        }
        for e in blue_edges {
            let u = e[0] as usize;
            let v = e[1] as usize;
            graph[1][u].push(v);
        }
        let mut queue: VecDeque<(usize, usize, usize)> = VecDeque::new();
        queue.push_back((0, 0, 0));
        queue.push_back((0, 1, 0));
        visited[0][0] = true;
        visited[1][0] = true;
        while let Some((u, c, d)) = queue.pop_front() {
            if res[u] == -1 {
                res[u] = d as i32;
            }
            for &v in &graph[c][u] {
                if !visited[1 - c][v] {
                    visited[1 - c][v] = true;
                    queue.push_back((v, 1 - c, d + 1));
                }
            }
        }
        res
    }
}

#[test]
fn test() {
    let n = 3;
    let red_edges = vec_vec_i32![[0, 1], [1, 2]];
    let blue_edges = vec_vec_i32![];
    let res = vec![0, 1, -1];
    assert_eq!(
        Solution::shortest_alternating_paths(n, red_edges, blue_edges),
        res
    );
    let n = 3;
    let red_edges = vec_vec_i32![[0, 1]];
    let blue_edges = vec_vec_i32![[2, 1]];
    let res = vec![0, 1, -1];
    assert_eq!(
        Solution::shortest_alternating_paths(n, red_edges, blue_edges),
        res
    );
    let n = 3;
    let red_edges = vec_vec_i32![[1, 0]];
    let blue_edges = vec_vec_i32![[2, 1]];
    let res = vec![0, -1, -1];
    assert_eq!(
        Solution::shortest_alternating_paths(n, red_edges, blue_edges),
        res
    );
    let n = 3;
    let red_edges = vec_vec_i32![[0, 1]];
    let blue_edges = vec_vec_i32![[1, 2]];
    let res = vec![0, 1, 2];
    assert_eq!(
        Solution::shortest_alternating_paths(n, red_edges, blue_edges),
        res
    );
    let n = 3;
    let red_edges = vec_vec_i32![[0, 1], [0, 2]];
    let blue_edges = vec_vec_i32![[1, 0]];
    let res = vec![0, 1, 1];
    assert_eq!(
        Solution::shortest_alternating_paths(n, red_edges, blue_edges),
        res
    );
}
