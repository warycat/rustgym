struct Solution;
use rustgym_util::*;

trait Postorder {
    fn postorder(self) -> Self;
}

impl Postorder for TreeLink {
    fn postorder(self) -> Self {
        if let Some(node) = self {
            let val = node.borrow().val;
            let left = node.borrow_mut().left.take();
            let right = node.borrow_mut().right.take();
            let left = left.postorder();
            let right = right.postorder();
            if left.is_none() && right.is_none() && val == 0 {
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
    fn prune_tree(root: TreeLink) -> TreeLink {
        root.postorder()
    }
}

#[test]
fn test() {
    let root = tree!(1, None, tree!(0, tree!(0), tree!(1)));
    let res = tree!(1, None, tree!(0, None, tree!(1)));
    assert_eq!(Solution::prune_tree(root), res);
    let root = tree!(
        1,
        tree!(0, tree!(0), tree!(0)),
        tree!(1, tree!(0), tree!(1))
    );
    let res = tree!(1, None, tree!(1, None, tree!(1)));
    assert_eq!(Solution::prune_tree(root), res);
    let root = tree!(
        1,
        tree!(1, tree!(1, tree!(0), None), tree!(1)),
        tree!(0, tree!(0), tree!(1))
    );
    let res = tree!(1, tree!(1, tree!(1), tree!(1)), tree!(0, None, tree!(1)));
    assert_eq!(Solution::prune_tree(root), res);
}
