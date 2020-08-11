struct Solution;
use rustgym_util::*;

trait Postorder {
    fn postorder(self, target: i32) -> Self;
}

impl Postorder for TreeLink {
    fn postorder(self, target: i32) -> Self {
        if let Some(node) = self {
            let val = node.borrow().val;
            let left = node.borrow_mut().left.take();
            let right = node.borrow_mut().right.take();
            let left = left.postorder(target);
            let right = right.postorder(target);
            if left.is_none() && right.is_none() && val == target {
                None
            } else {
                tree!(val, left, right)
            }
        } else {
            None
        }
    }
}

impl Solution {
    fn remove_leaf_nodes(root: TreeLink, target: i32) -> TreeLink {
        root.postorder(target)
    }
}

#[test]
fn test() {
    let root = tree!(1, tree!(2, tree!(2), None), tree!(3, tree!(2), tree!(4)));
    let target = 2;
    let res = tree!(1, None, tree!(3, None, tree!(4)));
    assert_eq!(Solution::remove_leaf_nodes(root, target), res);
    let root = tree!(1, tree!(3, tree!(3), tree!(2)), tree!(3));
    let target = 3;
    let res = tree!(1, tree!(3, None, tree!(2)), None);
    assert_eq!(Solution::remove_leaf_nodes(root, target), res);
    let root = tree!(1, tree!(2, tree!(2, tree!(2), None), None), None);
    let target = 2;
    let res = tree!(1);
    assert_eq!(Solution::remove_leaf_nodes(root, target), res);
}
