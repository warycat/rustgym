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
    fn inorder(link: Link, next: Link) -> Link {
        if let Some(node) = link.as_ref() {
            let mut node = node.borrow_mut();
            let left = node.left.take();
            let right = node.right.take();
            let res = Self::inorder(left, link.clone());
            node.right = Self::inorder(right, next);
            res
        } else {
            next
        }
    }
}

impl Solution {
    fn increasing_bst(root: Link) -> Link {
        TreeNode::inorder(root, None)
    }
}

#[test]
fn test() {
    let root = TreeNode::branch(
        5,
        TreeNode::branch(
            3,
            TreeNode::branch(2, TreeNode::leaf(1), None),
            TreeNode::leaf(4),
        ),
        TreeNode::branch(
            6,
            None,
            TreeNode::branch(8, TreeNode::leaf(7), TreeNode::leaf(9)),
        ),
    );
    let res = TreeNode::branch(
        1,
        None,
        TreeNode::branch(
            2,
            None,
            TreeNode::branch(
                3,
                None,
                TreeNode::branch(
                    4,
                    None,
                    TreeNode::branch(
                        5,
                        None,
                        TreeNode::branch(
                            6,
                            None,
                            TreeNode::branch(7, None, TreeNode::branch(8, None, TreeNode::leaf(9))),
                        ),
                    ),
                ),
            ),
        ),
    );
    assert_eq!(Solution::increasing_bst(root), res);
}
