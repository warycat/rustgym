struct Solution;
use rustgym_util::*;

trait Preorder {
    fn preorder(self, left: TreeLink, right: TreeLink) -> TreeLink;
}

impl Preorder for TreeLink {
    fn preorder(self, left: TreeLink, right: TreeLink) -> TreeLink {
        if let Some(node) = self {
            let left_tree = node.borrow_mut().left.take();
            let right_leaf = node.borrow_mut().right.take();
            node.borrow_mut().left = left;
            node.borrow_mut().right = right;
            left_tree.preorder(right_leaf, Some(node))
        } else {
            right
        }
    }
}

impl Solution {
    fn upside_down_binary_tree(root: TreeLink) -> TreeLink {
        root.preorder(None, None)
    }
}

#[test]
fn test() {
    let root = tree!(1, tree!(2, tree!(4), tree!(5)), tree!(3));
    let res = tree!(4, tree!(5), tree!(2, tree!(3), tree!(1)));
    assert_eq!(Solution::upside_down_binary_tree(root), res);
}
