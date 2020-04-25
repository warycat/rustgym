#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TreeNode {
    pub val: i32,
    pub left: TreeLink,
    pub right: TreeLink,
}

#[macro_export]
macro_rules! tree {
    ($e:expr) => {
        TreeNode::leaf($e)
    };
    ($e:expr, $l:expr, $r:expr) => {
        TreeNode::branch($e, $l, $r)
    };
}

pub type TreeLink = Option<Rc<RefCell<TreeNode>>>;

use std::cell::RefCell;
use std::rc::Rc;

impl TreeNode {
    pub fn branch(val: i32, left: TreeLink, right: TreeLink) -> TreeLink {
        Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
    }
    pub fn leaf(val: i32) -> TreeLink {
        Some(Rc::new(RefCell::new(TreeNode {
            val,
            left: None,
            right: None,
        })))
    }
}
