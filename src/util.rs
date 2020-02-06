#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: ListLink,
}

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: TreeLink,
    pub right: TreeLink,
}

pub type ListLink = Option<Box<ListNode>>;

impl ListNode {
    pub fn node(val: i32, next: ListLink) -> ListLink {
        Some(Box::new(ListNode { val, next }))
    }
}

#[macro_export]
macro_rules! list {
    () => {
        None
    };
    ($e:expr) => {
        ListNode::node($e, None)
    };
    ($e:expr, $($tail:tt)*) => {
        ListNode::node($e, list!($($tail)*))
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

#[macro_export]
macro_rules! tree {
    ($e:expr) => {
        TreeNode::leaf($e)
    };
    ($e:expr, $l:expr, $r:expr) => {
        TreeNode::branch($e, $l, $r)
    };
}

#[macro_export]
macro_rules! vec_string {
    ($($e:expr),*) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push((*$e).to_string());
            )*
            temp_vec
        }
    };
}

#[macro_export]
macro_rules! vec_vec_i32 {
    ($($e:expr),*) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($e.to_vec());
            )*
            temp_vec
        }
    };
}

#[macro_export]
macro_rules! vec_vec_string {
    ($($e:expr),*) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($e.iter().map(|s| (*s).to_string()).collect());
            )*
            temp_vec
        }
    };
}
