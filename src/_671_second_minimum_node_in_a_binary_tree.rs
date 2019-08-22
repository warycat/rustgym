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
    fn find_second_minimum_value(root: &Link, min: &mut Option<i32>) -> Option<i32> {
        if let Some(node) = root {
            let node = node.borrow();
            let left = &node.left;
            let right = &node.right;
            if min.is_none() {
                *min = Some(node.val);
            }
            if min.unwrap() == node.val {
                let min_left = Self::find_second_minimum_value(&left, min);
                let min_right = Self::find_second_minimum_value(&right, min);
                Self::option_min(min_left, min_right)
            } else {
                Some(node.val)
            }
        } else {
            None
        }
    }
    fn option_min(a: Option<i32>, b: Option<i32>) -> Option<i32> {
        match (a, b) {
            (Some(a), Some(b)) => Some(i32::min(a, b)),
            (Some(a), None) => Some(a),
            (None, Some(b)) => Some(b),
            (None, None) => None,
        }
    }
}

impl Solution {
    fn find_second_minimum_value(root: Link) -> i32 {
        let mut min = None;
        if let Some(second_min) = TreeNode::find_second_minimum_value(&root, &mut min) {
            second_min
        } else {
            -1
        }
    }
}

#[test]
fn test() {
    let root = TreeNode::branch(
        2,
        TreeNode::leaf(2),
        TreeNode::branch(5, TreeNode::leaf(5), TreeNode::leaf(7)),
    );
    assert_eq!(Solution::find_second_minimum_value(root), 5);
    let root = TreeNode::branch(2, TreeNode::leaf(2), TreeNode::leaf(2));
    assert_eq!(Solution::find_second_minimum_value(root), -1);
}
