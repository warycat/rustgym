struct Solution;
use rustgym_util::*;

trait SubTree {
    fn is_subtree(&self, t: &TreeLink) -> bool;
}
impl SubTree for TreeLink {
    fn is_subtree(&self, t: &TreeLink) -> bool {
        if self == t {
            return true;
        }
        if let Some(node) = self {
            let left = &node.borrow().left;
            let right = &node.borrow().right;
            return left.is_subtree(t) || right.is_subtree(t);
        }
        false
    }
}

impl Solution {
    fn is_subtree(s: TreeLink, t: TreeLink) -> bool {
        s.is_subtree(&t)
    }
}

#[test]
fn test() {
    let s = tree!(3, tree!(4, tree!(1), tree!(2)), tree!(5));
    let t = tree!(4, tree!(1), tree!(2));
    assert_eq!(Solution::is_subtree(s, t), true);
}
