struct Solution;
use rustgym_util::*;

trait MinDepth {
    fn min_depth(&self) -> usize;
}

impl MinDepth for TreeLink {
    fn min_depth(&self) -> usize {
        match self {
            None => 0,
            Some(node) => {
                let node = node.borrow();
                match (&node.left, &node.right) {
                    (None, None) => 1,
                    (Some(_), None) => TreeLink::min_depth(&node.left) + 1,
                    (None, Some(_)) => TreeLink::min_depth(&node.right) + 1,
                    (Some(_), Some(_)) => {
                        usize::min(
                            TreeLink::min_depth(&node.left),
                            TreeLink::min_depth(&node.right),
                        ) + 1
                    }
                }
            }
        }
    }
}

impl Solution {
    fn min_depth(root: TreeLink) -> i32 {
        root.min_depth() as i32
    }
}

#[test]
fn test() {
    let root = tree!(3, tree!(9), tree!(20, tree!(15), tree!(7)));
    assert_eq!(Solution::min_depth(root), 2);
}
