struct Solution {}

#[derive(PartialEq, Eq, Clone, Debug)]
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

impl Solution {
    fn invert_tree(root: Tree) -> Tree {
        if let Some(node) = &root {
            let mut node = node.borrow_mut();
            let left = node.left.take();
            let right = node.right.take();
            node.right = Solution::invert_tree(left);
            node.left = Solution::invert_tree(right);
        }
        root
    }
}

#[test]
fn test() {
    let input = TreeNode::branch(
        4,
        TreeNode::branch(2, TreeNode::leaf(1), TreeNode::leaf(3)),
        TreeNode::branch(7, TreeNode::leaf(6), TreeNode::leaf(9)),
    );
    let output = TreeNode::branch(
        4,
        TreeNode::branch(7, TreeNode::leaf(9), TreeNode::leaf(6)),
        TreeNode::branch(2, TreeNode::leaf(3), TreeNode::leaf(1)),
    );
    assert_eq!(Solution::invert_tree(input), output);
}
