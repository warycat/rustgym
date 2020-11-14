#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TreeNode {
    pub val: i32,
    pub left: TreeLink,
    pub right: TreeLink,
}
pub type TreeLink = Option<Rc<RefCell<TreeNode>>>;

#[macro_export]
macro_rules! tree {
    ($e:expr) => {
        TreeLink::leaf($e)
    };
    ($e:expr, $l:expr, $r:expr) => {
        TreeLink::branch($e, $l, $r)
    };
}

use std::cell::RefCell;
use std::rc::Rc;

pub trait TreeMaker {
    fn branch(val: i32, left: TreeLink, right: TreeLink) -> TreeLink {
        Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
    }
    fn leaf(val: i32) -> TreeLink {
        Some(Rc::new(RefCell::new(TreeNode {
            val,
            left: None,
            right: None,
        })))
    }
}

impl TreeMaker for TreeLink {}
