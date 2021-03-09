struct Solution;
use rustgym_util::*;

trait Inorder {
    fn inorder(&mut self, sum: &mut i32);
}

impl Inorder for TreeLink {
    fn inorder(&mut self, sum: &mut i32) {
        if let Some(node) = self {
            node.borrow_mut().right.inorder(sum);
            *sum += node.borrow().val;
            node.borrow_mut().val = *sum;
            node.borrow_mut().left.inorder(sum);
        }
    }
}

impl Solution {
    fn bst_to_gst(mut root: TreeLink) -> TreeLink {
        let mut sum = 0;
        root.inorder(&mut sum);
        root
    }
}

#[test]
fn test() {
    let root = tree!(
        4,
        tree!(1, tree!(0), tree!(2, None, tree!(3))),
        tree!(6, tree!(5), tree!(7, None, tree!(8)))
    );
    let res = tree!(
        30,
        tree!(36, tree!(36), tree!(35, None, tree!(33))),
        tree!(21, tree!(26), tree!(15, None, tree!(8)))
    );
    assert_eq!(Solution::bst_to_gst(root), res);
}
