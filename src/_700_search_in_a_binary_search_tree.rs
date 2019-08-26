struct Solution;

#[derive(Debug, PartialEq, Eq)]
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
    fn find(link: Link, val: i32) -> Link {
        if let Some(node) = link {
            let temp = node.clone();
            let node = node.borrow();
            if val == node.val {
                Some(temp)
            } else {
                if val < node.val {
                    Self::find(node.left.clone(), val)
                } else {
                    Self::find(node.right.clone(), val)
                }
            }
        } else {
            None
        }
    }
}

impl Solution {
    fn search_bst(root: Link, val: i32) -> Link {
        TreeNode::find(root, val)
    }
}

#[test]
fn test() {
    let root = TreeNode::branch(
        4,
        TreeNode::branch(2, TreeNode::leaf(1), TreeNode::leaf(3)),
        TreeNode::leaf(3),
    );
    let res = TreeNode::branch(2, TreeNode::leaf(1), TreeNode::leaf(3));
    assert_eq!(Solution::search_bst(root, 2), res);
}
