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
    fn merge_trees(t1: Link, t2: Link) -> Link {
        match (t1, t2) {
            (Some(n1), Some(n2)) => {
                let mut n1 = n1.borrow_mut();
                let mut n2 = n2.borrow_mut();
                TreeNode::branch(
                    n1.val + n2.val,
                    Self::merge_trees(n1.left.take(), n2.left.take()),
                    Self::merge_trees(n1.right.take(), n2.right.take()),
                )
            }
            (None, Some(n2)) => Some(n2),
            (Some(n1), None) => Some(n1),
            (None, None) => None,
        }
    }
}

#[test]
fn test() {
    let t1 = TreeNode::branch(
        1,
        TreeNode::branch(3, TreeNode::leaf(5), None),
        TreeNode::leaf(2),
    );
    let t2 = TreeNode::branch(
        2,
        TreeNode::branch(1, None, TreeNode::leaf(4)),
        TreeNode::branch(3, None, TreeNode::leaf(7)),
    );
    let res = TreeNode::branch(
        3,
        TreeNode::branch(4, TreeNode::leaf(5), TreeNode::leaf(4)),
        TreeNode::branch(5, None, TreeNode::leaf(7)),
    );
    assert_eq!(Solution::merge_trees(t1, t2), res);
}
