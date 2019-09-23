struct Solution;

struct TreeNode {
    val: i32,
    left: Tree,
    right: Tree,
}

use std::cell::RefCell;
use std::rc::Rc;

type Tree = Option<Rc<RefCell<TreeNode>>>;

impl TreeNode {
    fn branch(val: i32, left: Tree, right: Tree) -> Tree {
        Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
    }
    fn leaf(val: i32) -> Tree {
        Some(Rc::new(RefCell::new(TreeNode {
            val,
            left: None,
            right: None,
        })))
    }
}

trait Height {
    fn height(&self) -> usize;
}

impl Height for Tree {
    fn height(&self) -> usize {
        match self {
            None => 0,
            Some(node) => {
                let node = node.borrow();
                usize::max(node.left.height(), node.right.height()) + 1
            }
        }
    }
}

impl Solution {
    fn is_balanced_r(root: &Tree) -> bool {
        match root {
            None => true,
            Some(node) => {
                let node = node.borrow();
                let left = &node.left;
                let right = &node.right;
                let height_left = left.height();
                let height_right = right.height();
                if height_left == height_right
                    || height_left == height_right + 1
                    || height_left + 1 == height_right
                {
                    Solution::is_balanced_r(left) && Solution::is_balanced_r(right)
                } else {
                    false
                }
            }
        }
    }
    fn is_balanced(root: Tree) -> bool {
        Solution::is_balanced_r(&root)
    }
}

#[test]
fn test() {
    let a = TreeNode::branch(
        3,
        TreeNode::leaf(9),
        TreeNode::branch(20, TreeNode::leaf(15), TreeNode::leaf(7)),
    );
    assert_eq!(Solution::is_balanced(a), true);
    let b = TreeNode::branch(
        1,
        TreeNode::branch(
            2,
            TreeNode::branch(3, TreeNode::leaf(4), TreeNode::leaf(4)),
            TreeNode::leaf(3),
        ),
        TreeNode::leaf(2),
    );
    assert_eq!(Solution::is_balanced(b), false);
}
