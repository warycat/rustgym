struct Solution;
use crate::util::*;

trait PathSum {
    fn has_path_sum(&self, sum: i32) -> bool;
}

impl PathSum for TreeLink {
    fn has_path_sum(&self, sum: i32) -> bool {
        match self {
            None => sum == 0,
            Some(node) => {
                let node = node.borrow();
                match (&node.left, &node.right) {
                    (None, None) => sum == node.val,
                    (left, right) => {
                        left.has_path_sum(sum - node.val) || right.has_path_sum(sum - node.val)
                    }
                }
            }
        }
    }
}

impl Solution {
    fn has_path_sum(root: TreeLink, sum: i32) -> bool {
        if root.is_none() {
            return false;
        }
        root.has_path_sum(sum)
    }
}

#[test]
fn test() {
    let root = tree!(
        5,
        tree!(4, tree!(11, tree!(7), tree!(2)), None),
        tree!(8, tree!(13), tree!(4, None, tree!(1)))
    );
    assert_eq!(Solution::has_path_sum(root, 22), true);
}
