struct Solution;

struct TreeNode {
    val: i32,
    left: Link,
    right: Link,
}

use std::cell::RefCell;
use std::i32;
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
    fn inorder(link: &Link, prev: &mut Option<i32>, min: &mut i32) {
        if let Some(node) = link {
            let node = node.borrow();
            Self::inorder(&node.left, prev, min);
            if let Some(prev_val) = prev.as_mut() {
                *min = i32::min((node.val - *prev_val).abs(), *min);
                *prev_val = node.val;
            } else {
                *prev = Some(node.val);
            }
            Self::inorder(&node.right, prev, min);
        }
    }
}

impl Solution {
    fn get_minimum_difference(root: Link) -> i32 {
        let mut min = i32::MAX;
        let mut prev: Option<i32> = None;
        TreeNode::inorder(&root, &mut prev, &mut min);
        min
    }
}

#[test]
fn test() {
    let root = TreeNode::branch(1, None, TreeNode::branch(3, TreeNode::leaf(2), None));
    assert_eq!(Solution::get_minimum_difference(root), 1);
}
