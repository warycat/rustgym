//! # Install dependencies
//! ##  macos
//! ```bash
//! brew install graphviz
//! ```
//! ## linux
//! ```bash
//! apt install graphviz
//! ```
//! # Handy macros to build data for testing
//! ```
//! use rustgym_util::*;
//!
//! // singly linked list
//! let list = list!(1, 2, 3);
//! // binary tree
//! let root = tree!(1, tree!(2, tree!(3), tree!(4)), None);
//! // 1D vector of String
//! let names = vec_string!["Larry Fantasy", "Yinchu Xia"];
//! // 2D vector of String
//! let names_2d = vec_vec_string![["Larry", "Fantasy"], ["Yinchu", "Xia"]];
//! // 2D vector of i32
//! let matrix_i32 = vec_vec_i32![[1, 2], [3, 4]];
//! // 2D vector of char
//! let matrix_char = vec_vec_char![['a', 'b'], ['c', 'd']];
//! ```
//! # Boilerplate for singly linked list
//! Replace
//! ```
//! use rustgym_util::*;
//! ```
//! with
//! ```ignore
//! #[macro_export]
//! macro_rules! list {
//!     () => {
//!         None
//!     };
//!     ($e:expr) => {
//!         ListLink::link($e, None)
//!     };
//!     ($e:expr, $($tail:tt)*) => {
//!         ListLink::link($e, list!($($tail)*))
//!     };
//! }
//!
//! pub type ListLink = Option<Box<ListNode>>;
//!
//! pub trait ListMaker {
//!     fn link(val: i32, next: ListLink) -> ListLink {
//!         Some(Box::new(ListNode { val, next }))
//!     }
//! }
//!
//! impl ListMaker for ListLink {}
//! ```
//! when submitting to leetcode online judge.
//!
//! # Boilerplate for binary tree
//! Replace
//! ```
//! use rustgym_util::*;
//! ```
//! with
//! ```ignore
//! #[macro_export]
//! macro_rules! tree {
//!     ($e:expr) => {
//!         TreeLink::leaf($e)
//!     };
//!     ($e:expr, $l:expr, $r:expr) => {
//!         TreeLink::branch($e, $l, $r)
//!     };
//! }
//!
//! pub type TreeLink = Option<Rc<RefCell<TreeNode>>>;
//!
//! use std::cell::RefCell;
//! use std::rc::Rc;
//!
//! pub trait TreeMaker {
//!     fn branch(val: i32, left: TreeLink, right: TreeLink) -> TreeLink {
//!         Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
//!     }
//!     fn leaf(val: i32) -> TreeLink {
//!         Some(Rc::new(RefCell::new(TreeNode {
//!             val,
//!             left: None,
//!             right: None,
//!         })))
//!     }
//! }
//! ```
//! when submitting to leetcode online judge.
#[cfg_attr(test, macro_use)]
extern crate rustgym_util;

#[allow(dead_code)]
#[deny(clippy::all)]
#[allow(clippy::collapsible_if)]
#[allow(clippy::needless_range_loop)]
#[allow(clippy::too_many_arguments)]
mod leetcode;

#[allow(dead_code)]
#[deny(clippy::all)]
#[allow(clippy::collapsible_if)]
#[allow(clippy::needless_range_loop)]
#[allow(clippy::too_many_arguments)]
mod hackerrank;

#[allow(dead_code)]
#[deny(clippy::all)]
#[allow(clippy::collapsible_if)]
#[allow(clippy::needless_range_loop)]
#[allow(clippy::too_many_arguments)]
#[allow(clippy::while_let_on_iterator)]
pub mod advent_of_code_2020;
