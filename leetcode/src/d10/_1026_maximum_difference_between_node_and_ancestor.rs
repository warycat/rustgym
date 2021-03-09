struct Solution;
use rustgym_util::*;

trait Preorder {
    fn preorder(&self, parent: Option<(i32, i32)>, abs: &mut i32);
}

impl Preorder for TreeLink {
    fn preorder(&self, mut parent: Option<(i32, i32)>, abs: &mut i32) {
        if let Some(node) = self {
            let val = node.borrow().val;
            let left = &node.borrow().left;
            let right = &node.borrow().right;
            parent = if let Some((min, max)) = parent {
                *abs = (*abs).max((min - val).abs().max((max - val).abs()));
                Some((min.min(val), max.max(val)))
            } else {
                Some((val, val))
            };
            left.preorder(parent, abs);
            right.preorder(parent, abs);
        }
    }
}

impl Solution {
    fn max_ancestor_diff(root: TreeLink) -> i32 {
        let mut res = 0;
        root.preorder(None, &mut res);
        res
    }
}

#[test]
fn test() {
    let root = tree!(
        8,
        tree!(3, tree!(1), tree!(6, tree!(4), tree!(7))),
        tree!(10, None, tree!(14, tree!(13), None))
    );
    let res = 7;
    assert_eq!(Solution::max_ancestor_diff(root), res);
}
