struct Solution;
use rustgym_util::*;

trait Inorder {
    fn inorder(&mut self, sum: &mut i32);
}

impl Inorder for TreeLink {
    fn inorder(&mut self, sum: &mut i32) {
        if let Some(node) = self {
            let mut node = node.borrow_mut();
            node.right.inorder(sum);
            *sum += node.val;
            node.val = *sum;
            node.left.inorder(sum);
        }
    }
}

impl Solution {
    fn convert_bst(mut root: TreeLink) -> TreeLink {
        let mut sum = 0;
        root.inorder(&mut sum);
        root
    }
}

#[test]
fn test() {
    let root = tree!(5, tree!(2), tree!(13));
    let greater = tree!(18, tree!(20), tree!(13));
    assert_eq!(Solution::convert_bst(root), greater);
}
