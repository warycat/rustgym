struct Solution;
use std::collections::HashSet;

impl Solution {
    fn min_time(n: i32, edges: Vec<Vec<i32>>, has_apple: Vec<bool>) -> i32 {
        let n = n as usize;
        let mut graph = vec![HashSet::new(); n];
        for e in edges {
            let u = e[0] as usize;
            let v = e[1] as usize;
            graph[u].insert(v);
            graph[v].insert(u);
        }
        let mut visited = vec![false; n];
        visited[0] = true;
        Self::dfs(0, &mut visited, &graph, &has_apple)
    }

    fn dfs(u: usize, visited: &mut Vec<bool>, graph: &[HashSet<usize>], has_apple: &[bool]) -> i32 {
        let mut res = 0;
        for &v in &graph[u] {
            if !visited[v] {
                visited[v] = true;
                res += Self::dfs(v, visited, graph, has_apple);
            }
        }
        if u != 0 && (res != 0 || has_apple[u]) {
            res += 2;
        }
        res
    }
}

#[test]
fn test() {
    let n = 7;
    let edges = vec_vec_i32![[0, 1], [0, 2], [1, 4], [1, 5], [2, 3], [2, 6]];
    let has_apple = vec![false, false, true, false, true, true, false];
    let res = 8;
    assert_eq!(Solution::min_time(n, edges, has_apple), res);
    let n = 7;
    let edges = vec_vec_i32![[0, 1], [0, 2], [1, 4], [1, 5], [2, 3], [2, 6]];
    let has_apple = vec![false, false, true, false, false, true, false];
    let res = 6;
    assert_eq!(Solution::min_time(n, edges, has_apple), res);
    let n = 7;
    let edges = vec_vec_i32![[0, 1], [0, 2], [1, 4], [1, 5], [2, 3], [2, 6]];
    let has_apple = vec![false, false, false, false, false, false, false];
    let res = 0;
    assert_eq!(Solution::min_time(n, edges, has_apple), res);
    let n = 4;
    let edges = vec_vec_i32![[0, 1], [1, 2], [0, 3]];
    let has_apple = vec![true, true, true, true];
    let res = 6;
    assert_eq!(Solution::min_time(n, edges, has_apple), res);
}
