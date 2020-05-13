struct Solution;

impl Solution {
    fn tree_diameter(edges: Vec<Vec<i32>>) -> i32 {
        let n = edges.len() + 1;
        let mut graph = vec![vec![]; n];
        for e in edges {
            let u = e[0] as usize;
            let v = e[1] as usize;
            graph[u].push(v);
            graph[v].push(u);
        }
        let mut visited = vec![false; n];
        let mut res = 0;
        Self::dfs(0, &mut visited, &mut res, &graph);
        res as i32
    }

    fn dfs(u: usize, visited: &mut Vec<bool>, diameter: &mut usize, graph: &[Vec<usize>]) -> usize {
        visited[u] = true;
        let mut max_depth = 0;
        for &v in &graph[u] {
            if !visited[v] {
                let depth = Self::dfs(v, visited, diameter, graph);
                *diameter = (*diameter).max(depth + max_depth);
                max_depth = max_depth.max(depth);
            }
        }
        max_depth + 1
    }
}

#[test]
fn test() {
    let edges = vec_vec_i32![[0, 1], [0, 2]];
    let res = 2;
    assert_eq!(Solution::tree_diameter(edges), res);
    let edges = vec_vec_i32![[0, 1], [1, 2], [2, 3], [1, 4], [4, 5]];
    let res = 4;
    assert_eq!(Solution::tree_diameter(edges), res);
}
