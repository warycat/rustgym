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
    fn sum_root_to_leaf(link: &Link, parent_val: i32, sum: &mut i32) {
        if let Some(node) = link {
            let node = node.borrow();
            let left = &node.left;
            let right = &node.right;
            let val = node.val + parent_val * 2;
            if left.is_none() && right.is_none() {
                *sum += val;
            } else {
                Self::sum_root_to_leaf(left, val, sum);
                Self::sum_root_to_leaf(right, val, sum);
            }
        }
    }
}

impl Solution {
    fn sum_root_to_leaf(root: Link) -> i32 {
        let mut sum = 0;
        TreeNode::sum_root_to_leaf(&root, 0, &mut sum);
        sum
    }
}

#[test]
fn test() {
    let root = TreeNode::branch(
        1,
        TreeNode::branch(0, TreeNode::leaf(0), TreeNode::leaf(1)),
        TreeNode::branch(1, TreeNode::leaf(0), TreeNode::leaf(1)),
    );
    assert_eq!(Solution::sum_root_to_leaf(root), 22);
}
