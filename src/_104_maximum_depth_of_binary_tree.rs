struct Solution;

struct TreeNode {
    val: i32,
    left: Tree,
    right: Tree,
}

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

type Tree = Option<Rc<RefCell<TreeNode>>>;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    fn max_depth_r(root: &Tree) -> i32 {
        if let Some(node) = root {
            let node = node.borrow();
            return 1 + i32::max(
                Solution::max_depth_r(&node.left),
                Solution::max_depth_r(&node.right),
            );
        }
        0
    }

    fn max_depth(root: Tree) -> i32 {
        Solution::max_depth_r(&root)
    }
}

#[test]
fn test() {
    let p = TreeNode::branch(
        3,
        TreeNode::leaf(9),
        TreeNode::branch(20, TreeNode::leaf(15), TreeNode::leaf(7)),
    );
    assert_eq!(Solution::max_depth(p), 3);
}
