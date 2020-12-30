struct Solution;
use rustgym_util::*;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

#[derive(Default)]
struct Graph {
    edges: HashMap<i32, Vec<i32>>,
    nodes: HashMap<i32, bool>,
}

type Edge = (i32, bool);

trait Preorder {
    fn preorder(&self, parent: Option<i32>, graph: &mut Graph);
}

impl Preorder for TreeLink {
    fn preorder(&self, parent: Option<i32>, graph: &mut Graph) {
        if let Some(node) = self {
            let val = node.borrow().val;
            let left = &node.borrow().left;
            let right = &node.borrow().right;
            let is_leaf = left.is_none() && right.is_none();
            graph.add_node(val, is_leaf);
            if let Some(parent) = parent {
                graph.add_edge(parent, val);
                graph.add_edge(val, parent);
            }
            left.preorder(Some(val), graph);
            right.preorder(Some(val), graph);
        }
    }
}

impl Graph {
    fn add_node(&mut self, u: i32, is_leaf: bool) {
        *self.nodes.entry(u).or_default() = is_leaf;
    }
    fn add_edge(&mut self, u: i32, v: i32) {
        self.edges.entry(u).or_default().push(v);
    }
}

impl Solution {
    fn find_closest_leaf(root: TreeLink, k: i32) -> i32 {
        let mut graph: Graph = Graph::default();
        root.preorder(None, &mut graph);
        let mut queue: VecDeque<i32> = VecDeque::new();
        let mut visited: HashSet<i32> = HashSet::new();
        visited.insert(k);
        queue.push_back(k);
        while let Some(u) = queue.pop_front() {
            if graph.nodes[&u] {
                return u;
            } else {
                for &v in &graph.edges[&u] {
                    if visited.insert(v) {
                        queue.push_back(v);
                    }
                }
            }
        }
        0
    }
}

#[test]
fn test() {
    let root = tree!(1, tree!(3), tree!(2));
    let k = 1;
    let res = 3;
    assert_eq!(Solution::find_closest_leaf(root, k), res);
    let root = tree!(1);
    let k = 1;
    let res = 1;
    assert_eq!(Solution::find_closest_leaf(root, k), res);
    let root = tree!(
        1,
        tree!(2, tree!(4, tree!(5, tree!(6), None), None), None),
        tree!(3)
    );
    let k = 1;
    let res = 3;
    assert_eq!(Solution::find_closest_leaf(root, k), res);
}
