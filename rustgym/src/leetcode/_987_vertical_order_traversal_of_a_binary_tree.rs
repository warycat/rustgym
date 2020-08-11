struct Solution;
use rustgym_util::*;
use std::cmp::Reverse;
use std::collections::BTreeMap;
use std::collections::BinaryHeap;

type Nodes = BTreeMap<i32, BTreeMap<i32, BinaryHeap<Reverse<i32>>>>;

trait VerticalOrder {
    fn vertical_order(self) -> Vec<Vec<i32>>;
}

impl VerticalOrder for Nodes {
    fn vertical_order(self) -> Vec<Vec<i32>> {
        let n = self.len();
        let mut nodes = self;
        let mut res: Vec<Vec<i32>> = vec![vec![]; n];
        for (i, col) in nodes.values_mut().enumerate() {
            for row in col.values_mut() {
                while let Some(Reverse(smallest)) = row.pop() {
                    res[i].push(smallest);
                }
            }
        }
        res
    }
}

trait Preorder {
    fn preorder(&self, x: i32, y: i32, nodes: &mut Nodes);
}

impl Preorder for TreeLink {
    fn preorder(&self, x: i32, y: i32, nodes: &mut Nodes) {
        if let Some(node) = self {
            let node = node.borrow();
            nodes
                .entry(x)
                .or_default()
                .entry(y)
                .or_default()
                .push(Reverse(node.val));
            node.left.preorder(x - 1, y + 1, nodes);
            node.right.preorder(x + 1, y + 1, nodes);
        }
    }
}

impl Solution {
    fn vertical_traversal(root: TreeLink) -> Vec<Vec<i32>> {
        let mut nodes: Nodes = BTreeMap::new();
        root.preorder(0, 0, &mut nodes);
        nodes.vertical_order()
    }
}

#[test]
fn test() {
    let root = tree!(3, tree!(9), tree!(20, tree!(15), tree!(7)));
    let res = vec![vec![9], vec![3, 15], vec![20], vec![7]];
    assert_eq!(Solution::vertical_traversal(root), res);
    let root = tree!(
        1,
        tree!(2, tree!(4), tree!(5)),
        tree!(3, tree!(6), tree!(7))
    );
    let res = vec![vec![4], vec![2], vec![1, 5, 6], vec![3], vec![7]];
    assert_eq!(Solution::vertical_traversal(root), res);
}
