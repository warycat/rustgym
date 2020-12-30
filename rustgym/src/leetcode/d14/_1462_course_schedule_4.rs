struct Solution;
use std::collections::HashSet;

impl Solution {
    fn check_if_prerequisite(
        n: i32,
        prerequisites: Vec<Vec<i32>>,
        queries: Vec<Vec<i32>>,
    ) -> Vec<bool> {
        let n = n as usize;
        let mut graph: Vec<HashSet<usize>> = vec![HashSet::new(); n];
        for edge in prerequisites {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            graph[u].insert(v);
        }
        let mut closures: Vec<HashSet<usize>> = vec![HashSet::new(); n];
        for i in 0..n {
            let mut visited = vec![false; n];
            Self::dfs(i, &mut visited, &mut closures, &graph, i);
        }
        queries
            .into_iter()
            .map(|v| closures[v[0] as usize].contains(&(v[1] as usize)))
            .collect()
    }

    fn dfs(
        u: usize,
        visited: &mut Vec<bool>,
        closures: &mut Vec<HashSet<usize>>,
        graph: &[HashSet<usize>],
        start: usize,
    ) {
        if !visited[u] {
            visited[u] = true;
            for &v in &graph[u] {
                closures[start].insert(v);
                Self::dfs(v, visited, closures, graph, start);
            }
        }
    }
}

#[test]
fn test() {
    let n = 2;
    let prerequisites = vec_vec_i32![[1, 0]];
    let queries = vec_vec_i32![[0, 1], [1, 0]];
    let res = vec![false, true];
    assert_eq!(
        Solution::check_if_prerequisite(n, prerequisites, queries),
        res
    );
    let n = 2;
    let prerequisites = vec_vec_i32![];
    let queries = vec_vec_i32![[1, 0], [0, 1]];
    let res = vec![false, false];
    assert_eq!(
        Solution::check_if_prerequisite(n, prerequisites, queries),
        res
    );
    let n = 3;
    let prerequisites = vec_vec_i32![[1, 2], [1, 0], [2, 0]];
    let queries = vec_vec_i32![[1, 0], [1, 2]];
    let res = vec![true, true];
    assert_eq!(
        Solution::check_if_prerequisite(n, prerequisites, queries),
        res
    );
    let n = 3;
    let prerequisites = vec_vec_i32![[1, 0], [2, 0]];
    let queries = vec_vec_i32![[0, 1], [2, 0]];
    let res = vec![false, true];
    assert_eq!(
        Solution::check_if_prerequisite(n, prerequisites, queries),
        res
    );
    let n = 5;
    let prerequisites = vec_vec_i32![[0, 1], [1, 2], [2, 3], [3, 4]];
    let queries = vec_vec_i32![[0, 4], [4, 0], [1, 3], [3, 0]];
    let res = vec![true, false, true, false];
    assert_eq!(
        Solution::check_if_prerequisite(n, prerequisites, queries),
        res
    );
}
