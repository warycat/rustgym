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

trait PathSum {
    fn has_path_sum(&self, sum: i32) -> bool;
}

impl PathSum for Tree {
    fn has_path_sum(&self, sum: i32) -> bool {
        match self {
            None => sum == 0,
            Some(node) => {
                let node = node.borrow();
                match (&node.left, &node.right) {
                    (None, None) => sum == node.val,
                    (left, right) => {
                        left.has_path_sum(sum - node.val) || right.has_path_sum(sum - node.val)
                    }
                }
            }
        }
    }
}

impl Solution {
    fn has_path_sum(root: Tree, sum: i32) -> bool {
        if root.is_none() {
            return false;
        }
        root.has_path_sum(sum)
    }
}

#[test]
fn test() {
    let root = TreeNode::branch(
        5,
        TreeNode::branch(
            4,
            TreeNode::branch(11, TreeNode::leaf(7), TreeNode::leaf(2)),
            None,
        ),
        TreeNode::branch(
            8,
            TreeNode::leaf(13),
            TreeNode::branch(4, None, TreeNode::leaf(1)),
        ),
    );
    assert_eq!(Solution::has_path_sum(root, 22), true);
}
