struct Solution;
use rustgym_util::*;

trait MaxDepth {
    fn max_depth(&self) -> i32;
}

impl MaxDepth for TreeLink {
    fn max_depth(&self) -> i32 {
        if let Some(node) = self {
            let node = node.borrow();
            return 1 + i32::max(node.left.max_depth(), node.right.max_depth());
        }
        0
    }
}

impl Solution {
    fn max_depth(root: TreeLink) -> i32 {
        root.max_depth()
    }
}

#[test]
fn test() {
    let p = tree!(3, tree!(9), tree!(20, tree!(15), tree!(7)));
    assert_eq!(Solution::max_depth(p), 3);
}
