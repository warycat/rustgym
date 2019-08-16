struct Solution;

#[derive(Debug, PartialEq, Eq)]
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
    fn is_subtree(s: Link, t: Link) -> bool {
        Solution::is_subtree_r(s, &t)
    }

    fn is_subtree_r(s: Link, t: &Link) -> bool {
        if s == *t {
            return true;
        }
        if let Some(node) = s {
            let left: Link = node.borrow_mut().left.take();
            let right: Link = node.borrow_mut().right.take();
            return Solution::is_subtree_r(left, t) || Solution::is_subtree_r(right, t);
        }
        false
    }
}

#[test]
fn test() {
    let s: Link = TreeNode::branch(
        3,
        TreeNode::branch(4, TreeNode::leaf(1), TreeNode::leaf(2)),
        TreeNode::leaf(5),
    );
    let t: Link = TreeNode::branch(4, TreeNode::leaf(1), TreeNode::leaf(2));
    assert_eq!(Solution::is_subtree(s, t), true);
}
