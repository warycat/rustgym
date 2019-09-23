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

impl Solution {
    fn tree2str(t: Tree) -> String {
        if let Some(node) = t {
            let mut node = node.borrow_mut();
            let left = node.left.take();
            let right = node.right.take();
            match (&left, &right) {
                (Some(_), Some(_)) => format!(
                    "{}({})({})",
                    node.val,
                    Solution::tree2str(left),
                    Solution::tree2str(right)
                ),
                (Some(_), None) => format!("{}({})", node.val, Solution::tree2str(left)),
                (None, Some(_)) => format!("{}()({})", node.val, Solution::tree2str(right)),
                (None, None) => format!("{}", node.val),
            }
        } else {
            "".to_string()
        }
    }
}

#[test]
fn test() {
    let t = TreeNode::branch(
        1,
        TreeNode::branch(2, TreeNode::leaf(4), None),
        TreeNode::leaf(3),
    );
    assert_eq!(Solution::tree2str(t), "1(2(4))(3)".to_string());
}
