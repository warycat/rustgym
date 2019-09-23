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
    fn inorder(root: &Link, v: &mut Vec<i32>, target: i32) {
        if let Some(node) = root {
            let node = node.borrow();
            let left = &node.left;
            let right = &node.right;
            Self::inorder(left, v, target);
            v.push(node.val);
            Self::inorder(right, v, target);
        }
    }
}

impl Solution {
    fn find_target(root: Link, k: i32) -> bool {
        let mut v = vec![];
        TreeNode::inorder(&root, &mut v, k);
        let n = v.len();
        let mut l = 0;
        let mut r = n - 1;
        while l < r {
            let sum = v[l] + v[r];
            if sum > k {
                r -= 1;
            } else if sum < k {
                l += 1;
            } else {
                return true;
            }
        }
        false
    }
}

#[test]
fn test() {
    let root = TreeNode::branch(
        5,
        TreeNode::branch(3, TreeNode::leaf(2), TreeNode::leaf(4)),
        TreeNode::branch(6, None, TreeNode::leaf(7)),
    );
    assert_eq!(Solution::find_target(root, 9), true);
}
