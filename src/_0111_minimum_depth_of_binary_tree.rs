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

trait MinDepth {
    fn min_depth(&self) -> usize;
}

impl MinDepth for Tree {
    fn min_depth(&self) -> usize {
        match self {
            None => 0,
            Some(node) => {
                let node = node.borrow();
                match (&node.left, &node.right) {
                    (None, None) => 1,
                    (Some(_), None) => Tree::min_depth(&node.left) + 1,
                    (None, Some(_)) => Tree::min_depth(&node.right) + 1,
                    (Some(_), Some(_)) => {
                        usize::min(Tree::min_depth(&node.left), Tree::min_depth(&node.right)) + 1
                    }
                }
            }
        }
    }
}

impl Solution {
    fn min_depth(root: Tree) -> i32 {
        root.min_depth() as i32
    }
}

#[test]
fn test() {
    let root = TreeNode::branch(
        3,
        TreeNode::leaf(9),
        TreeNode::branch(20, TreeNode::leaf(15), TreeNode::leaf(7)),
    );
    assert_eq!(Solution::min_depth(root), 2);
}
