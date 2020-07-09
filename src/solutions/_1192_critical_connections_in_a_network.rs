struct Solution;

#[derive(Debug, Eq, PartialEq, Clone)]
struct Node {
    id: usize,
    discovery: usize,
    lowest: usize,
    on_stack: bool,
}

impl Node {
    fn new(id: usize) -> Node {
        let discovery = std::usize::MAX;
        let lowest = std::usize::MAX;
        let on_stack = false;
        Node {
            id,
            discovery,
            lowest,
            on_stack,
        }
    }
}

impl Solution {
    fn critical_connections(n: i32, connections: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = n as usize;
        let mut graph: Vec<Vec<usize>> = vec![vec![]; n];
        for connection in connections {
            let u = connection[0] as usize;
            let v = connection[1] as usize;
            graph[u].push(v);
            graph[v].push(u);
        }
        let mut time = 0;
        let mut nodes: Vec<Node> = (0..n).map(Node::new).collect();
        let mut res = vec![];
        Self::dfs(0, 0, &mut time, &mut nodes, &mut res, &graph);
        res
    }

    fn dfs(
        u: usize,
        parent: usize,
        time: &mut usize,
        nodes: &mut Vec<Node>,
        edges: &mut Vec<Vec<i32>>,
        graph: &[Vec<usize>],
    ) {
        nodes[u].discovery = *time;
        nodes[u].lowest = *time;
        *time += 1;
        for &v in &graph[u] {
            if v == parent {
                continue;
            }
            if nodes[v].discovery == std::usize::MAX {
                Self::dfs(v, u, time, nodes, edges, graph);
                nodes[u].lowest = nodes[u].lowest.min(nodes[v].lowest);
                if nodes[v].lowest > nodes[u].discovery {
                    edges.push(vec![u as i32, v as i32]);
                }
            } else {
                nodes[u].lowest = nodes[u].lowest.min(nodes[v].discovery);
            }
        }
    }
}

#[test]
fn test() {
    let n = 4;
    let connections = vec_vec_i32![[0, 1], [1, 2], [2, 0], [1, 3]];
    let res = vec_vec_i32![[1, 3]];
    assert_eq!(Solution::critical_connections(n, connections), res);
    let n = 5;
    let connections = vec_vec_i32![[1, 0], [2, 0], [3, 0], [4, 1], [4, 2], [4, 0]];
    let res = vec_vec_i32![[0, 3]];
    assert_eq!(Solution::critical_connections(n, connections), res);
}
