struct Solution;
use std::collections::HashSet;

struct Graph {
    edges: Vec<HashSet<usize>>,
    nodes: Vec<i32>,
    n: usize,
}

impl Graph {
    fn new(n: usize) -> Self {
        let edges: Vec<HashSet<usize>> = vec![HashSet::new(); n];
        let nodes: Vec<i32> = vec![0; n];
        Graph { edges, nodes, n }
    }

    fn insert_edge(&mut self, u: usize, v: usize) {
        self.edges[u].insert(v);
    }

    fn dfs(&mut self, u: usize, color: i32) -> bool {
        if self.nodes[u] == 0 {
            self.nodes[u] = color;
            for v in self.edges[u].clone() {
                if !self.dfs(v, -color) {
                    return false;
                }
            }
        } else {
            return self.nodes[u] == color;
        }
        true
    }
}

impl Solution {
    fn is_bipartite(graph: Vec<Vec<i32>>) -> bool {
        let n = graph.len();
        let mut g = Graph::new(n);
        for u in 0..n {
            for &v in &graph[u] {
                g.insert_edge(u, v as usize);
            }
        }
        for u in 0..n {
            if g.nodes[u] == 0 && !g.dfs(u, 1) {
                return false;
            }
        }
        true
    }
}

#[test]
fn test() {
    let graph = vec![vec![1, 3], vec![0, 2], vec![1, 3], vec![0, 2]];
    let res = true;
    assert_eq!(Solution::is_bipartite(graph), res);
    let graph = vec![vec![1, 2, 3], vec![0, 2], vec![0, 1, 3], vec![0, 2]];
    let res = false;
    assert_eq!(Solution::is_bipartite(graph), res);
    let graph = vec![
        vec![],
        vec![2, 4, 6],
        vec![1, 4, 8, 9],
        vec![7, 8],
        vec![1, 2, 8, 9],
        vec![6, 9],
        vec![1, 5, 7, 8, 9],
        vec![3, 6, 9],
        vec![2, 3, 4, 6, 9],
        vec![2, 4, 5, 6, 7, 8],
    ];
    let res = false;
    assert_eq!(Solution::is_bipartite(graph), res);
}
