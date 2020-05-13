struct Solution;

impl Solution {
    fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let mut path: Vec<i32> = vec![];
        let n = graph.len();
        Self::dfs(0, &mut path, &mut res, &graph, n);
        res
    }

    fn dfs(u: i32, path: &mut Vec<i32>, paths: &mut Vec<Vec<i32>>, graph: &[Vec<i32>], n: usize) {
        path.push(u);
        if u as usize == n - 1 {
            paths.push(path.clone());
        } else {
            for &v in &graph[u as usize] {
                Self::dfs(v, path, paths, graph, n);
            }
        }
        path.pop();
    }
}

#[test]
fn test() {
    let graph = vec_vec_i32![[1, 2], [3], [3], []];
    let res = vec_vec_i32![[0, 1, 3], [0, 2, 3]];
    assert_eq!(Solution::all_paths_source_target(graph), res);
}
