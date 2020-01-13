struct Solution;
use crate::util::*;

impl Solution {
    fn is_symmetric_r(p: &TreeLink, q: &TreeLink) -> bool {
        match (p, q) {
            (Some(p), Some(q)) => {
                let p = p.borrow();
                let q = q.borrow();
                p.val == q.val
                    && Solution::is_symmetric_r(&p.left, &q.right)
                    && Solution::is_symmetric_r(&p.right, &q.left)
            }
            (None, None) => true,
            _ => false,
        }
    }

    fn is_symmetric(root: TreeLink) -> bool {
        if let Some(node) = root {
            let node = node.borrow();
            return Solution::is_symmetric_r(&node.left, &node.right);
        }
        true
    }
}

#[test]
fn test() {
    let q = tree!(
        1,
        tree!(2, tree!(1), tree!(1)),
        tree!(2, tree!(1), tree!(1))
    );
    assert_eq!(Solution::is_symmetric(q), true)
}
