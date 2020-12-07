struct Solution;
use rustgym_util::*;

trait PathSum {
    fn has_path_sum(&self, sum: i32) -> bool;
}

impl PathSum for TreeLink {
    fn has_path_sum(&self, sum: i32) -> bool {
        if let Some(node) = self {
            let node = node.borrow();
            let val = node.val;
            let left = &node.left;
            let right = &node.right;
            if left.is_none() && right.is_none() {
                sum == val
            } else {
                right.has_path_sum(sum - val) || left.has_path_sum(sum - val)
            }
        } else {
            false
        }
    }
}

impl Solution {
    fn has_path_sum(root: TreeLink, sum: i32) -> bool {
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
    let sum = 22;
    let res = true;
    assert_eq!(Solution::has_path_sum(root, sum), res);
    let root = tree!(1, tree!(2), None);
    let sum = 1;
    let res = false;
    assert_eq!(Solution::has_path_sum(root, sum), res);
}
