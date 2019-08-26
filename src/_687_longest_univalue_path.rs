struct Solution;

struct TreeNode {
    val: i32,
    left: Link,
    right: Link,
}

use std::cell::RefCell;
use std::rc::Rc;

type Link = Option<Rc<RefCell<TreeNode>>>;

impl TreeNode {
    fn branch(val: i32, left: Link, right: Link) -> Link {
        Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
    }
    fn leaf(val: i32) -> Link {
        Some(Rc::new(RefCell::new(TreeNode {
            val,
            left: None,
            right: None,
        })))
    }
    fn longest_univalue_path_to_parent(root: &Link, max: &mut i32, parent_val: i32) -> i32 {
        if let Some(node) = root {
            let node = node.borrow();
            let left = Self::longest_univalue_path_to_parent(&node.left, max, node.val);
            let right = Self::longest_univalue_path_to_parent(&node.right, max, node.val);
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
    fn longest_univalue_path(root: Link) -> i32 {
        let mut max = 0;
        TreeNode::longest_univalue_path_to_parent(&root, &mut max, 0);
        max
    }
}

#[test]
fn test() {
    let root = TreeNode::branch(
        1,
        TreeNode::branch(4, TreeNode::leaf(4), TreeNode::leaf(4)),
        TreeNode::branch(5, None, TreeNode::leaf(5)),
    );
    assert_eq!(Solution::longest_univalue_path(root), 2);
    let root = TreeNode::branch(3, TreeNode::leaf(3), None);
    assert_eq!(Solution::longest_univalue_path(root), 1);
}
