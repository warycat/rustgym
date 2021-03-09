struct Solution;
use rustgym_util::*;

trait Postorder {
    fn postorder(&self) -> (usize, TreeLink);
}

impl Postorder for TreeLink {
    fn postorder(&self) -> (usize, TreeLink) {
        use std::cmp::Ordering::*;
        if let Some(node) = self {
            let node = node.borrow();
            let left = &node.left;
            let right = &node.right;
            let (left_depth, left_tree) = left.postorder();
            let (right_depth, rigth_tree) = right.postorder();
            match left_depth.cmp(&right_depth) {
                Equal => (left_depth + 1, self.clone()),
                Less => (right_depth + 1, rigth_tree),
                Greater => (left_depth + 1, left_tree),
            }
        } else {
            (0, None)
        }
    }
}

impl Solution {
    fn subtree_with_all_deepest(root: TreeLink) -> TreeLink {
        root.postorder().1
    }
}

#[test]
fn test() {
    let root = tree!(
        3,
        tree!(5, tree!(6), tree!(2, tree!(7), tree!(4))),
        tree!(1, tree!(0), tree!(8))
    );
    let res = tree!(2, tree!(7), tree!(4));
    assert_eq!(Solution::subtree_with_all_deepest(root), res);
}
