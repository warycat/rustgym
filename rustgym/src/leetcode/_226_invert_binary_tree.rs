struct Solution;
use rustgym_util::*;

impl Solution {
    fn invert_tree(root: TreeLink) -> TreeLink {
        if let Some(node) = &root {
            let mut node = node.borrow_mut();
            let left = node.left.take();
            let right = node.right.take();
            node.right = Self::invert_tree(left);
            node.left = Self::invert_tree(right);
        }
        root
    }
}

#[test]
fn test() {
    let input = tree!(
        4,
        tree!(2, tree!(1), tree!(3)),
        tree!(7, tree!(6), tree!(9))
    );
    let output = tree!(
        4,
        tree!(7, tree!(9), tree!(6)),
        tree!(2, tree!(3), tree!(1))
    );
    assert_eq!(Solution::invert_tree(input), output);
}
