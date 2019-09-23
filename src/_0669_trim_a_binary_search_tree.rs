struct Solution;

#[derive(Debug, PartialEq, Eq, Clone)]
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
}

impl Solution {
    fn trim_bst(root: Link, l: i32, r: i32) -> Link {
        if let Some(node) = root.clone() {
            let mut node = node.borrow_mut();
            let left = node.left.take();
            let right = node.right.take();
            if node.val > r {
                return Self::trim_bst(left, l, r);
            }
            if node.val < l {
                return Self::trim_bst(right, l, r);
            }
            node.left = Self::trim_bst(left, l, r);
            node.right = Self::trim_bst(right, l, r);
            root
        } else {
            None
        }
    }
}

#[test]
fn test() {
    let root = TreeNode::branch(1, TreeNode::leaf(0), TreeNode::leaf(2));
    let res = TreeNode::branch(1, None, TreeNode::leaf(2));
    assert_eq!(Solution::trim_bst(root, 1, 2), res);
    let root = TreeNode::branch(
        3,
        TreeNode::branch(0, None, TreeNode::branch(2, TreeNode::leaf(1), None)),
        TreeNode::leaf(4),
    );
    let res = TreeNode::branch(3, TreeNode::branch(2, TreeNode::leaf(1), None), None);
    assert_eq!(Solution::trim_bst(root, 1, 3), res);
}
