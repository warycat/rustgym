struct Solution;

#[derive(PartialEq, Eq, Clone, Debug)]
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

use std::cell::RefCell;
use std::rc::Rc;

type Tree = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    fn is_same_tree(p: Tree, q: Tree) -> bool {
        p == q
    }
}

#[test]
fn test() {
    let q = TreeNode::branch(
        1,
        TreeNode::branch(2, TreeNode::leaf(1), TreeNode::leaf(1)),
        TreeNode::branch(2, TreeNode::leaf(1), TreeNode::leaf(1)),
    );

    let p = TreeNode::branch(
        1,
        TreeNode::branch(2, TreeNode::leaf(1), TreeNode::leaf(1)),
        TreeNode::branch(2, TreeNode::leaf(1), TreeNode::leaf(1)),
    );

    assert_eq!(Solution::is_same_tree(p, q), true);
}
