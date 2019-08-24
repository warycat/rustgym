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
    fn sum_of_left_leaves(link: &Link) -> i32 {
        let mut sum = 0;
        if let Some(node) = link {
            let node = node.borrow();
            let left = &node.left;
            let right = &node.right;
            if let Some(left_node) = left {
                let left_node = left_node.borrow();
                if left_node.left.is_none() && left_node.right.is_none() {
                    sum += left_node.val;
                } else {
                    sum += Self::sum_of_left_leaves(left);
                }
            }
            sum += Self::sum_of_left_leaves(right);
        }
        sum
    }
}

impl Solution {
    fn sum_of_left_leaves(root: Link) -> i32 {
        TreeNode::sum_of_left_leaves(&root)
    }
}

#[test]
fn test() {
    let root = TreeNode::branch(
        3,
        TreeNode::leaf(9),
        TreeNode::branch(20, TreeNode::leaf(15), TreeNode::leaf(7)),
    );
    assert_eq!(Solution::sum_of_left_leaves(root), 24);
}
