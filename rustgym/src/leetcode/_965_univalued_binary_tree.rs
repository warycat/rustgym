struct Solution;
use rustgym_util::*;

trait Preorder {
    fn is_unival(&self) -> bool;
    fn preorder(&self, val: &mut Option<i32>) -> bool;
}

impl Preorder for TreeLink {
    fn is_unival(&self) -> bool {
        let mut val: Option<i32> = None;
        self.preorder(&mut val)
    }
    fn preorder(&self, val: &mut Option<i32>) -> bool {
        if let Some(node) = self {
            let node = node.borrow();
            let node_val = node.val;
            if let Some(val) = val {
                if *val != node_val {
                    return false;
                }
            } else {
                *val = Some(node_val);
            }
            node.left.preorder(val) && node.right.preorder(val)
        } else {
            true
        }
    }
}

impl Solution {
    fn is_unival_tree(root: TreeLink) -> bool {
        root.is_unival()
    }
}

#[test]
fn test() {
    let root = tree!(1, tree!(1, tree!(1), tree!(1)), tree!(1, None, tree!(1)));
    assert_eq!(Solution::is_unival_tree(root), true);
    let root = tree!(2, tree!(2, tree!(5), tree!(2)), tree!(2));
    assert_eq!(Solution::is_unival_tree(root), false);
}
