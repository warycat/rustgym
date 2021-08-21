use std::collections::HashSet;

#[derive(Debug, Eq, PartialEq)]
struct Node {
    id: usize,
    discovery: usize,
    lowest: usize,
    on_stack: bool,
}

impl Node {
    pub fn new(id: usize) -> Self {
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

    fn visit(&mut self, time: usize) {
        self.discovery = time;
    }

    fn is_visited(&self) -> bool {
        self.discovery != std::usize::MAX
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum EdgeKind {
    TreeEdge,
    BackEdge,
    CrossEdge,
    ForwardEdge,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Edge {
    pub tail: usize,
    pub head: usize,
    pub kind: EdgeKind,
}

impl Edge {
    fn new(tail: usize, head: usize, kind: EdgeKind) -> Self {
        Edge { tail, head, kind }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Graph {
    pub n: usize,
    pub edges: Vec<Edge>,
    adj: Vec<HashSet<usize>>,
    nodes: Vec<Node>,
    stack: Vec<usize>,
    time: usize,
}

impl Graph {
    pub fn new(n: usize) -> Self {
        let time = 0;
        let adj = vec![HashSet::new(); n];
        let nodes = (0..n).map(Node::new).collect();
        let edges = vec![];
        let stack = vec![];
        Graph {
            n,
            edges,
            adj,
            nodes,
            stack,
            time,
        }
    }

    pub fn init_with_edges(&mut self, edges: Vec<Vec<i32>>, directed: bool) {
        for edge in edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            self.adj[u].insert(v);
            if !directed {
                self.adj[v].insert(u);
            }
        }
    }

    pub fn dfs(&mut self, u: usize, parent: usize) {
        self.nodes[u].visit(self.time);
        self.time += 1;
        let neighbours: Vec<usize> = self.adj[u].iter().copied().collect();
        for v in neighbours {
            if v == parent {
                continue;
            }
            if !self.nodes[v].is_visited() {
                self.edges.push(Edge::new(u, v, EdgeKind::TreeEdge));
                self.dfs(v, u);
            } else {
                self.edges.push(Edge::new(u, v, EdgeKind::BackEdge));
            }
        }
    }

    pub fn travase(&mut self) {
        for u in 0..self.n {
            if !self.nodes[u].is_visited() {
                self.dfs(u, u);
            }
        }
    }
}
