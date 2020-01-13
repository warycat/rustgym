struct Solution;
use crate::util::*;

impl Solution {
    fn max_depth_r(root: &TreeLink) -> i32 {
        if let Some(node) = root {
            let node = node.borrow();
            return 1 + i32::max(
                Solution::max_depth_r(&node.left),
                Solution::max_depth_r(&node.right),
            );
        }
        0
    }

    fn max_depth(root: TreeLink) -> i32 {
        Solution::max_depth_r(&root)
    }
}

#[test]
fn test() {
    let p = tree!(3, tree!(9), tree!(20, tree!(15), tree!(7)));
    assert_eq!(Solution::max_depth(p), 3);
}
