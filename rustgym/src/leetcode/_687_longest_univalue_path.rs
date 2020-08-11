struct Solution;
use rustgym_util::*;

trait LongestUnivaluePath {
    fn longest_univalue_path(&self, max: &mut i32, parent_val: i32) -> i32;
}

impl LongestUnivaluePath for TreeLink {
    fn longest_univalue_path(&self, max: &mut i32, parent_val: i32) -> i32 {
        if let Some(node) = self {
            let node = node.borrow();
            let left = Self::longest_univalue_path(&node.left, max, node.val);
            let right = Self::longest_univalue_path(&node.right, max, node.val);
            *max = i32::max(left + right, *max);
            if parent_val == node.val {
                i32::max(left, right) + 1
            } else {
                0
            }
        } else {
            0
        }
    }
}

impl Solution {
    fn longest_univalue_path(root: TreeLink) -> i32 {
        let mut max = 0;
        root.longest_univalue_path(&mut max, 0);
        max
    }
}

#[test]
fn test() {
    let root = tree!(1, tree!(4, tree!(4), tree!(4)), tree!(5, None, tree!(5)));
    assert_eq!(Solution::longest_univalue_path(root), 2);
    let root = tree!(3, tree!(3), None);
    assert_eq!(Solution::longest_univalue_path(root), 1);
}
