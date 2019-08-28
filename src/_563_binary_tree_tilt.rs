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
    fn dfs(link: &Link, tilt: &mut i32) -> i32 {
        if let Some(node) = link {
            let node = node.borrow();
            let left = &node.left;
            let right = &node.right;
            let left_sum = Self::dfs(left, tilt);
            let right_sum = Self::dfs(right, tilt);
            *tilt = *tilt + (left_sum - right_sum).abs();
            node.val + left_sum + right_sum
        } else {
            0
        }
    }
}

impl Solution {
    fn find_tilt(root: Link) -> i32 {
        let mut tilt = 0;
        TreeNode::dfs(&root, &mut tilt);
        tilt
    }
}

#[test]
fn test() {
    let root = TreeNode::branch(1, TreeNode::leaf(2), TreeNode::leaf(3));
    assert_eq!(Solution::find_tilt(root), 1);
}
