struct Solution;
use rustgym_util::*;

trait Boundry {
    fn left_boundry(&self, nodes: &mut Vec<i32>);
    fn bottom_boundry(&self, nodes: &mut Vec<i32>);
    fn right_boundry(&self, nodes: &mut Vec<i32>);
}

impl Boundry for TreeLink {
    fn left_boundry(&self, nodes: &mut Vec<i32>) {
        if let Some(node) = self {
            let node = node.borrow();
            let left = &node.left;
            let right = &node.right;
            if left.is_some() || right.is_some() {
                nodes.push(node.val);
            }
            if left.is_some() {
                left.left_boundry(nodes);
            } else {
                right.left_boundry(nodes);
            }
        }
    }
    fn bottom_boundry(&self, nodes: &mut Vec<i32>) {
        if let Some(node) = self {
            let node = node.borrow();
            let left = &node.left;
            let right = &node.right;
            if left.is_none() && right.is_none() {
                nodes.push(node.val);
            } else {
                left.bottom_boundry(nodes);
                right.bottom_boundry(nodes);
            }
        }
    }
    fn right_boundry(&self, nodes: &mut Vec<i32>) {
        if let Some(node) = self {
            let node = node.borrow();
            let left = &node.left;
            let right = &node.right;
            if right.is_some() {
                right.right_boundry(nodes);
            } else {
                left.right_boundry(nodes);
            }
            if left.is_some() || right.is_some() {
                nodes.push(node.val);
            }
        }
    }
}

impl Solution {
    fn boundary_of_binary_tree(root: TreeLink) -> Vec<i32> {
        let mut res: Vec<i32> = vec![];
        if let Some(node) = root {
            let node = node.borrow();
            res.push(node.val);
            node.left.left_boundry(&mut res);
            node.left.bottom_boundry(&mut res);
            node.right.bottom_boundry(&mut res);
            node.right.right_boundry(&mut res);
        } else {
            return vec![];
        }
        res
    }
}

#[test]
fn test() {
    let root = tree!(1, None, tree!(2, tree!(3), tree!(4)));
    let res = vec![1, 3, 4, 2];
    assert_eq!(Solution::boundary_of_binary_tree(root), res);
    let root = tree!(
        1,
        tree!(2, tree!(4), tree!(5, tree!(7), tree!(8))),
        tree!(3, tree!(6, tree!(9), tree!(10)), None)
    );
    let res = vec![1, 2, 4, 7, 8, 9, 10, 6, 3];
    assert_eq!(Solution::boundary_of_binary_tree(root), res);
}
