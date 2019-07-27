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

use std::cell::RefCell;
use std::rc::Rc;

type Tree = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    fn is_symmetric_r(p: &Tree, q: &Tree) -> bool {
        match (p, q) {
            (Some(p), Some(q)) => {
                let p = p.borrow();
                let q = q.borrow();
                p.val == q.val
                    && Solution::is_symmetric_r(&p.left, &q.right)
                    && Solution::is_symmetric_r(&p.right, &q.left)
            }
            (None, None) => true,
            _ => false,
        }
    }

    fn is_symmetric(root: Tree) -> bool {
        if let Some(node) = root {
            let node = node.borrow();
            return Solution::is_symmetric_r(&node.left, &node.right);
        }
        true
    }
}

#[test]
fn test() {
    let q = TreeNode::branch(
        1,
        TreeNode::branch(2, TreeNode::leaf(1), TreeNode::leaf(1)),
        TreeNode::branch(2, TreeNode::leaf(1), TreeNode::leaf(1)),
    );
    assert_eq!(Solution::is_symmetric(q), true)
}
