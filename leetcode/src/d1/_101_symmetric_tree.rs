struct Solution;
use rustgym_util::*;

trait Symmetric {
    fn is_symmetric(&self) -> bool;
    fn is_mirror(&self, right: &TreeLink) -> bool;
}

impl Symmetric for TreeLink {
    fn is_symmetric(&self) -> bool {
        if let Some(node) = self {
            let node = node.borrow();
            node.left.is_mirror(&node.right)
        } else {
            true
        }
    }

    fn is_mirror(&self, right: &TreeLink) -> bool {
        match (self, right) {
            (Some(p), Some(q)) => {
                let p = p.borrow();
                let q = q.borrow();
                p.val == q.val && p.left.is_mirror(&q.right) && p.right.is_mirror(&q.left)
            }
            (None, None) => true,
            _ => false,
        }
    }
}

impl Solution {
    fn is_symmetric(root: TreeLink) -> bool {
        root.is_symmetric()
    }
}

#[test]
fn test() {
    let q = tree!(
        1,
        tree!(2, tree!(1), tree!(1)),
        tree!(2, tree!(1), tree!(1))
    );
    let res = true;
    assert_eq!(Solution::is_symmetric(q), res);
}
