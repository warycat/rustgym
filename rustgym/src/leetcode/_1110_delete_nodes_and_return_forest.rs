struct Solution;
use rustgym_util::*;
use std::collections::HashSet;
use std::iter::FromIterator;

trait Postorder {
    fn postorder(self, nodes: &HashSet<i32>) -> (TreeLink, Vec<TreeLink>);
}

impl Postorder for TreeLink {
    fn postorder(self, nodes: &HashSet<i32>) -> (TreeLink, Vec<TreeLink>) {
        if let Some(node) = self {
            let val = node.borrow_mut().val;
            let left = node.borrow_mut().left.take();
            let right = node.borrow_mut().right.take();
            let (left_root, left_forest) = left.postorder(nodes);
            let (right_root, right_forest) = right.postorder(nodes);
            let mut forest = vec![];
            for t in left_forest {
                forest.push(t);
            }
            for t in right_forest {
                forest.push(t);
            }
            if nodes.contains(&val) {
                if left_root.is_some() {
                    forest.push(left_root);
                }
                if right_root.is_some() {
                    forest.push(right_root);
                }
                (None, forest)
            } else {
                node.borrow_mut().left = left_root;
                node.borrow_mut().right = right_root;
                (Some(node), forest)
            }
        } else {
            (None, vec![])
        }
    }
}

impl Solution {
    fn del_nodes(root: TreeLink, to_delete: Vec<i32>) -> Vec<TreeLink> {
        let nodes = HashSet::from_iter(to_delete);
        let mut res = vec![];
        let (root, forest) = root.postorder(&nodes);
        if root.is_some() {
            res.push(root);
        }
        for t in forest {
            res.push(t);
        }
        res
    }
}

#[test]
fn test() {
    let root = tree!(
        1,
        tree!(2, tree!(4), tree!(5)),
        tree!(3, tree!(6), tree!(7))
    );
    let to_delete = vec![3, 5];
    let res = vec![tree!(1, tree!(2, tree!(4), None), None), tree!(6), tree!(7)];
    assert_eq!(Solution::del_nodes(root, to_delete), res);
}
